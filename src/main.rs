extern crate termion;
extern crate signal_hook;
extern crate mpd;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc;
use std::env;

use shellbird::event::{MpdRequest, Event, ScreenRequest, ComponentRequest};
use shellbird::music::{mpd_sender, mpd_listener};
use shellbird::init_stdin_thread;
use shellbird::screen;
use shellbird::signals;
use shellbird::styles;
use shellbird::command_line::CommandLine;
use shellbird::mode::Mode;
use shellbird::screen::Screen;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::{clear, cursor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let genre_path = match parse_args() {
        Some(path) => path,
        None => {
            println!("usage: shellbird <path/to/genres.txt>");
            return Ok(())
        }
    };

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
    styles::load_style_tree_async(&genre_path, tx.clone());

    loop {
        write!(stdout, "{}", clear::All)?;

        screens[sel].draw();

        if matches!(mode, Mode::Command) || matches!(mode, Mode::Search) {
            write!(stdout, "{}", command_line).unwrap();
        }

        stdout.flush().unwrap();

        match rx.recv()? {
            Event::Quit => break,
            Event::Mode(m) => {
                mode = m;
                command_line.clear();
                command_line.mode(mode.clone());
            },
            Event::SwitchScreen(i) => sel = i,
            Event::MpdRequest(r) => {
                mpd_tx.send(r).unwrap();
            },
            Event::ScreenRequest(r) => screens[sel].handle_request(&r, tx.clone()),
            Event::StyleTreeLoaded(style) => if let Some(style) = style {
                    tx.send(Event::UpdateRootStyleMenu(style.children())).unwrap();
            },
            Event::Input(key) => match key {
                Key::Char(':') => tx.send(Event::Mode(Mode::Command)).unwrap(),
                Key::Char('/') => tx.send(Event::Mode(Mode::Search)).unwrap(),
                Key::Esc => tx.send(Event::Mode(Mode::TUI)).unwrap(),
                Key::Backspace => if let Some(event) = command_line.back() {
                    tx.send(event).unwrap();
                },
                Key::Char(c) => if let Some(event) = command_line.add(c) {
                    tx.send(event).unwrap();
                    tx.send(Event::Mode(Mode::TUI)).unwrap();
                },
                _ => (),
            },
            event => for screen in screens.iter_mut() {
                screen.update(&event, tx.clone())
            },
        }
    }

    write!(stdout, "{}{}", cursor::Restore, clear::All).unwrap();

    Ok(())
}

fn keybinds() -> HashMap<String, Event> {
    let mut keybinds: HashMap<String, Event> = HashMap::new();

    keybinds.insert(String::from("j"), Event::ScreenRequest(ScreenRequest::ComponentRequest(ComponentRequest::Next)));
    keybinds.insert(String::from("k"), Event::ScreenRequest(ScreenRequest::ComponentRequest(ComponentRequest::Prev)));
    keybinds.insert(String::from("h"), Event::ScreenRequest(ScreenRequest::FocusPrev));
    keybinds.insert(String::from("l"), Event::ScreenRequest(ScreenRequest::FocusNext));
    keybinds.insert(String::from(" "), Event::ScreenRequest(ScreenRequest::ComponentRequest(ComponentRequest::Select)));
    keybinds.insert(String::from("p"), Event::MpdRequest(MpdRequest::TogglePause));
    keybinds.insert(String::from("1"), Event::SwitchScreen(0));
    keybinds.insert(String::from("2"), Event::SwitchScreen(1));
    keybinds.insert(String::from("3"), Event::SwitchScreen(2));
    keybinds.insert(String::from("4"), Event::SwitchScreen(3));
    keybinds.insert(String::from("5"), Event::SwitchScreen(4));
    keybinds.insert(String::from("q"), Event::Quit);
    keybinds.insert(String::from("c"), Event::MpdRequest(MpdRequest::ClearQueue));
    keybinds.insert(String::from(":"), Event::Mode(Mode::Command));
    keybinds.insert(String::from("/"), Event::Mode(Mode::Search));
    keybinds.insert(String::from("gg"), Event::ScreenRequest(ScreenRequest::ComponentRequest(ComponentRequest::GoToTop)));
    keybinds.insert(String::from("G"), Event::ScreenRequest(ScreenRequest::ComponentRequest(ComponentRequest::GoToBottom)));

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

fn parse_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        None
    } else {
        Some(args[1].clone())
    }
}
