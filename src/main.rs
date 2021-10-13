extern crate termion;
extern crate clap_v3 as clap;
extern crate signal_hook;
extern crate mpd;
extern crate home;

use std::io::{self, Write, BufRead, BufReader};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::path::Path;
use std::fs::File;

use shellbird::GlobalState;
use shellbird::event::*;
use shellbird::music::{mpd_sender, mpd_listener};
use shellbird::screen;
use shellbird::signals;
use shellbird::styles;
use shellbird::command_line::{self, CommandLine};
use shellbird::screen::Screen;

use termion::raw::IntoRawMode;
use termion::{clear, cursor};
use termion::input::TermRead;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Sean D. <s.digirolamo218@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    genres: String,
    sbrc: Option<String>,
    #[clap(short)]
    debug: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut state = GlobalState {
        style_tree: None,
        library: Vec::new(),
    };

    let mut sel = "Default".to_string();
    let mut screens = init_screens();

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    let (tx, rx) = mpsc::channel();

    let mut command_line = CommandLine::new(tx.clone());
    let mpd_tx =  mpd_sender::init_mpd_sender_thread("127.0.0.1", "6600", tx.clone());

    init_stdin_thread(tx.clone());
    run_sbrc(opts.sbrc, tx.clone());
    mpd_listener::init_mpd_listener_thread("127.0.0.1", "6600", tx.clone());
    signals::init_listener(tx.clone());
    styles::load_style_tree_async(&opts.genres, tx.clone());

    let mut redraw = true;

    loop {
        if redraw {
            write!(stdout, "{}", clear::All)?;

            if let Some(screen) = screens.get(&sel) {
                screen.draw();
            }

            command_line.draw();

            stdout.flush().unwrap();
        }

        redraw = true;

        let e = rx.recv()?;

        if opts.debug {
            eprintln!("{:?}", e);
        }

        match e {
            Event::BindKey(key, e) => command_line.bind(key, e.to_event()),
            Event::ToApp(e) => match e {
                AppEvent::Quit => break,
                AppEvent::Database(tracks) => {
                    if let Some(tree) = &mut state.style_tree {
                        tree.set_tracks(tracks.clone());
                    }

                    tx.send(Event::ToGlobal(GlobalEvent::Database(tracks))).unwrap();
                },
                AppEvent::SwitchScreen(name) => sel = name.clone(),
                AppEvent::StyleTreeLoaded(tree) => {
                    state.style_tree = tree;
                    tx.send(
                        Event::ToGlobal(GlobalEvent::UpdateRootStyleMenu)
                    ).unwrap();
                },
                _ => (),
            },
            Event::ToCommandLine(e) => command_line.handle(&e, tx.clone()),
            Event::ToScreen(e) => if let Some(screen) = screens.get_mut(&sel) {
                screen.handle_screen(&e, tx.clone());
            },
            Event::ToGlobal(e) => {
                for screen in screens.values_mut() {
                    screen.handle_global(&state, &e, tx.clone())
                }
            },
            Event::ToFocus(e) => if let Some(screen) = screens.get_mut(&sel) {
                screen.handle_focus(&state, &e, tx.clone());
            },
            Event::ToMpd(e) => mpd_tx.send(e).unwrap(),
            _ => (),
        }
    }

    write!(stdout, "{}{}", cursor::Restore, clear::All).unwrap();

    Ok(())
}

fn init_screens() -> HashMap<String, Screen> {
    let mut ret = HashMap::new();

    let screen = screen::new_now_playing_screen();
    ret.insert(screen.name().to_string(), screen);

    let screen = screen::new_queue_screen();
    ret.insert(screen.name().to_string(), screen);

    let screen = screen::new_playlist_view_screen();
    ret.insert(screen.name().to_string(), screen);

    let screen = screen::new_library_view_screen();
    ret.insert(screen.name().to_string(), screen);

    let screen = screen::new_style_view_screen();
    ret.insert(screen.name().to_string(), screen);

    ret
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
