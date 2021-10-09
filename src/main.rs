extern crate termion;
extern crate clap_v3 as clap;
extern crate signal_hook;
extern crate mpd;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

use shellbird::event::*;
use shellbird::music::{mpd_sender, mpd_listener};
use shellbird::screen;
use shellbird::signals;
use shellbird::styles;
use shellbird::command_line::CommandLine;
use shellbird::mode::Mode;
use shellbird::screen::Screen;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::{clear, cursor};
use termion::input::TermRead;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Sean D. <s.digirolamo218@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    genres: String,
    #[clap(short)]
    debug: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut mode = Mode::TUI;
    let mut command_line = CommandLine::new(keybinds());

    let mut sel = 0;
    let mut screens = init_screens();

    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    let (tx, rx) = mpsc::channel();

    let mpd_tx =  mpd_sender::init_mpd_sender_thread("127.0.0.1", "6600");

    init_stdin_thread(tx.clone());
    mpd_listener::init_mpd_listener_thread("127.0.0.1", "6600", tx.clone());
    signals::init_listener(tx.clone());
    styles::load_style_tree_async(&opts.genres, tx.clone());

    loop {
        write!(stdout, "{}", clear::All)?;

        screens[sel].draw();

        if matches!(mode, Mode::Command) || matches!(mode, Mode::Search) {
            write!(stdout, "{}", command_line).unwrap();
        }

        stdout.flush().unwrap();

        let e = rx.recv()?;

        if opts.debug {
            eprintln!("{:?}", e);
        }

        match e {
            Event::ToApp(e) => match e {
                AppEvent::Input(key) => match key {
                    Key::Char(':') => tx.send(Event::ToApp(AppEvent::Mode(Mode::Command))).unwrap(),
                    Key::Char('/') => tx.send(Event::ToApp(AppEvent::Mode(Mode::Search))).unwrap(),
                    Key::Esc => tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap(),
                    Key::Backspace => if let Some(event) = command_line.back() {
                        tx.send(event).unwrap();
                    },
                    Key::Char(c) => if let Some(event) = command_line.add(c) {
                        tx.send(event).unwrap();
                        tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap();
                    },
                    _ => (),
                },
                AppEvent::Mode(m) => {
                    mode = m;
                    command_line.clear();
                    command_line.mode(mode.clone());
                },
                AppEvent::Quit => break,
                AppEvent::SwitchScreen(i) => sel = i,
                AppEvent::StyleTreeLoaded(style) => {
                    if let Some(style) = style {
                        tx.send(
                            Event::ToGlobal(GlobalEvent::UpdateRootStyleMenu(
                                    style.children()
                            ))
                        ).unwrap();
                    }
                },
                _ => (),
            },
            Event::ToScreen(e) => screens[sel].handle_screen(&e, tx.clone()),
            Event::ToGlobal(e) => {
                for screen in screens.iter_mut() {
                    screen.handle_global(&e, tx.clone())
                }
            },
            Event::ToFocus(e) => screens[sel].handle_focus(&e, tx.clone()),
            Event::ToMpd(e) => mpd_tx.send(e).unwrap(),
            _ => (),
        }
    }

    write!(stdout, "{}{}", cursor::Restore, clear::All).unwrap();

    Ok(())
}

fn keybinds() -> HashMap<String, Event> {
    let mut keybinds: HashMap<String, Event> = HashMap::new();

    keybinds.insert(String::from(" "), Event::ToFocus(FocusEvent::Select));
    keybinds.insert(String::from("j"), Event::ToFocus(FocusEvent::Next));
    keybinds.insert(String::from("k"), Event::ToFocus(FocusEvent::Prev));
    keybinds.insert(String::from("gg"), Event::ToFocus(FocusEvent::GoToTop));
    keybinds.insert(String::from("G"), Event::ToFocus(FocusEvent::GoToBottom));
    keybinds.insert(String::from("h"), Event::ToScreen(ScreenEvent::FocusPrev));
    keybinds.insert(String::from("l"), Event::ToScreen(ScreenEvent::FocusNext));
    keybinds.insert(String::from("p"), Event::ToMpd(MpdEvent::TogglePause));
    keybinds.insert(String::from("c"), Event::ToMpd(MpdEvent::ClearQueue));
    keybinds.insert(String::from("1"), Event::ToApp(AppEvent::SwitchScreen(0)));
    keybinds.insert(String::from("2"), Event::ToApp(AppEvent::SwitchScreen(1)));
    keybinds.insert(String::from("3"), Event::ToApp(AppEvent::SwitchScreen(2)));
    keybinds.insert(String::from("4"), Event::ToApp(AppEvent::SwitchScreen(3)));
    keybinds.insert(String::from("5"), Event::ToApp(AppEvent::SwitchScreen(4)));
    keybinds.insert(String::from("q"), Event::ToApp(AppEvent::Quit));
    keybinds.insert(String::from(":"), Event::ToApp(AppEvent::Mode(Mode::Command)));
    keybinds.insert(String::from("/"), Event::ToApp(AppEvent::Mode(Mode::Search)));

    keybinds
}

fn init_screens() -> Vec<Screen> {
    vec![
        screen::new_now_playing_screen(),
        screen::new_queue_screen(),
        screen::new_playlist_view_screen(),
        screen::new_library_view_screen(),
        screen::new_style_view_screen(),
    ]
}

fn init_stdin_thread(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys() {
            if let Ok(key) = key {
                tx.send(Event::ToApp(AppEvent::Input(key))).unwrap();
            }
        }
    });
}
