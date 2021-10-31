extern crate termion;
extern crate clap_v3 as clap;
extern crate signal_hook;
extern crate mpd;
extern crate home;

use std::io::{self, Write, BufRead, BufReader};
use std::collections::{VecDeque, HashMap};
use std::sync::mpsc;
use std::thread;
use std::path::Path;
use std::fs::File;

use shellbird::GlobalState;
use shellbird::event::*;
use shellbird::music::{mpd_sender, mpd_listener};
use shellbird::signals;
use shellbird::styles;
use shellbird::command_line::{self, CommandLine};
use shellbird::components::{Splitter, Components, Component, MoveFocusResult};

use termion::raw::IntoRawMode;
use termion::{clear, cursor};
use termion::input::TermRead;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Sean D. <s.digirolamo218@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    genres: Option<String>,
    sbrc: Option<String>,
    layout: Option<String>,
    #[clap(short)]
    debug: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut state = GlobalState::new();

    let mut components = init_components(get_layout_path(opts.layout));

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    let (tx, rx) = mpsc::channel();

    let mut command_line = CommandLine::new(tx.clone());
    let mpd_tx =  mpd_sender::init_mpd_sender_thread("127.0.0.1", "6600", tx.clone());

    init_stdin_thread(tx.clone());
    run_sbrc(opts.sbrc, tx.clone());
    mpd_listener::init_mpd_listener_thread("127.0.0.1", "6600", tx.clone());
    signals::init_listener(tx.clone());

    if let Some(path) = get_genre_path(opts.genres) {
        styles::load_style_tree_async(&path, tx.clone());
    }

    loop {
        command_line.draw();

        stdout.flush().unwrap();

        let e = rx.recv()?;

        if opts.debug {
            eprintln!("{:?}", e);
        }

        match e {
            Event::BindKey(key, e) => command_line.bind(key, e.to_event()),
            Event::ToComponent(name, e) => {
                if let Some(c) = components.get_mut(&name) {
                    c.handle(&state, &e, tx.clone());
                }
            },
            Event::ToApp(e) => match e {
                AppEvent::Quit => break,
                AppEvent::Error(s) => eprintln!("{}", s),
                AppEvent::ClearScreen => print!("{}", clear::All),
                AppEvent::DrawScreen => send_draw_screen(&state.screen, &components, &tx),
                AppEvent::LostMpdConnection => {
                    state.library = Vec::new();
                    tx.send(Event::ToAllComponents(ComponentEvent::LostMpdConnection)).unwrap();
                },
                AppEvent::Database(tracks) => {
                    if let Some(tree) = &mut state.style_tree {
                        tree.set_tracks(tracks.clone());
                    }

                    state.library = tracks.clone();

                    tx.send(Event::ToAllComponents(ComponentEvent::Database(tracks))).unwrap();
                },
                AppEvent::SwitchScreen(name) => {
                    state.screen = name.clone();
                    tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                },
                AppEvent::StyleTreeLoaded(tree) => {
                    state.style_tree = tree;
                    tx.send(
                        Event::ToAllComponents(ComponentEvent::UpdateRootStyleMenu)
                    ).unwrap();
                },
                AppEvent::Resize => tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap(),
            },
            Event::ToCommandLine(e) => command_line.handle(&e, tx.clone()),
            Event::ToScreen(e) => match e {
                ScreenEvent::FocusNext => {
                    focus_next(&state.screen, &mut components);
                    tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                },
                ScreenEvent::FocusPrev => {
                    focus_prev(&state.screen, &mut components);
                    tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                },
                ScreenEvent::NeedsRedraw(name) => {
                    if screen_contains(&state.screen, &name, &components) {
                        tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                    }
                },
            },
            Event::ToAllComponents(e) => {
                for c in components.values_mut() {
                    c.handle(&state, &e, tx.clone())
                }
            },
            Event::ToFocus(e) => {
                let focus = focus(&state.screen, &components).to_string();
                if let Some(c) = components.get_mut(&focus) {
                    c.handle(&state, &e, tx.clone());
                }
            },
            Event::ToMpd(e) => mpd_tx.send(e).unwrap(),
            _ => (),
        }
    }

    write!(stdout, "{}{}", cursor::Restore, clear::All).unwrap();

    Ok(())
}

fn init_components(path: Option<String>) -> HashMap<String, Components> {
    if let Some(path) = path {
        match shellbird::layout_config::load(&path) {
            Ok(map) => map,
            _ => HashMap::new(),
        }
    } else {
        HashMap::new()
    }
}

fn init_stdin_thread(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys() {
            if let Ok(key) = key {
                tx.send(
                    Event::ToCommandLine(CommandLineEvent::Input(key))
                ).unwrap();
            }
        }
    });
}

fn run_sbrc(path_override: Option<String>, tx: mpsc::Sender<Event>) {
    if let Some(path) = get_sbrc(path_override) {
        let sbrc = File::open(path).unwrap();
        let reader = BufReader::new(sbrc);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            match command_line::run_headless(&line, tx.clone()) {
                Ok(_) => (),
                _ => tx.send(
                    Event::ToCommandLine(
                        CommandLineEvent::SbrcError(i + 1, line.to_string())
                    )
                ).unwrap(),
            }
        }
    } else {
        tx.send(Event::ToCommandLine(CommandLineEvent::SbrcNotFound)).unwrap();
    }
}

fn get_sbrc(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/sbrc");
            home
        };

        let homedir = {
            home.push(".sbrc");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/sbrc");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}

fn get_layout_path(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/layout.json");
            home
        };

        let homedir = {
            home.push(".sblayout.json");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/layout.json");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}

fn get_genre_path(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/genres.txt");
            home
        };

        let homedir = {
            home.push(".sbgenres.txt");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/genres.txt");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}

fn send_draw_screen(
    screen: &str,
    components: &HashMap<String, Components>,
    tx: &mpsc::Sender<Event>
) {
    let (w, h) = termion::terminal_size().unwrap();
    let h = h - 1;
    if let Some(_) = components.get(screen) {
        tx.send(Event::ToComponent(
            screen.to_string(),
            ComponentEvent::Draw(1, 1, w, h, focus(screen, components)),
        )).unwrap();
    } else {
        tx.send(Event::ToApp(AppEvent::ClearScreen)).unwrap();
    }
}

fn focus<'a>(
    screen: &'a str,
    components: &'a HashMap<String, Components>
) -> String {
    let stack = construct_focus_stack(screen, components);

    let key = stack.back().unwrap().to_string();

    match components.get(&key) {
        Some(Components::Splitter(s)) => match s.focus() {
            Some(focus) => focus.to_string(),
            None => s.name().to_string(),
        },
        _ => key.to_string(),
    }
}

fn focus_next(screen: &str, components: &mut HashMap<String, Components>) {
    let mut stack = construct_focus_stack(screen, components);

    let mut res = MoveFocusResult::Fail;
    while res == MoveFocusResult::Fail {
        if let Some(key) = stack.pop_back() {
            let c = match components.get_mut(&key) {
                Some(Components::Splitter(s)) => s,
                _ => break,
            };

            res = c.next();
        } else {
            break;
        }
    }
}

fn focus_prev(screen: &str, components: &mut HashMap<String, Components>) {
    let mut stack = construct_focus_stack(screen, components);

    let mut res = MoveFocusResult::Fail;
    while res == MoveFocusResult::Fail {
        if let Some(key) = stack.pop_back() {
            let c = match components.get_mut(&key) {
                Some(Components::Splitter(s)) => s,
                _ => break,
            };

            res = c.prev();
        } else {
            break;
        }
    }
}

fn construct_focus_stack(
    screen: &str,
    components: &HashMap<String, Components>
) -> VecDeque<String> {
    let mut stack: VecDeque<String> = VecDeque::new();
    stack.push_back(screen.to_string());
    loop {
        let back = stack.back().unwrap().to_string();
        if let Some(Components::Splitter(s)) = components.get(&back) {
            if let Some(focus) = s.focus() {
                if let Some(Components::Splitter(s)) = components.get(focus) {
                    stack.push_back(s.name().to_string());
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break
        }
    }

    stack
}

fn screen_contains(
    screen: &str,
    key: &str,
    components: &HashMap<String, Components>
) -> bool {
    if screen == key {
        true
    } else {
        match components.get(screen) {
            Some(c) => splitter_contains(c, key, components),
            None => false,
        }
    }
}

fn splitter_contains(
    component: &Components,
    key: &str,
    components: &HashMap<String, Components>,
) -> bool {
    match component {
        Components::Splitter(splitter) => {
            if splitter.name() == key {
                true
            } else {
                for child in splitter.children() {
                    if let Some(component) = components.get(child) {
                        if splitter_contains(component, key, components) {
                            return true
                        }
                    }
                }

                false
            }
        },
        _ => component.name() == key,
    }
}
