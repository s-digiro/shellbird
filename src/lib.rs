extern crate mpd;
extern crate termion;

pub mod event;
pub mod components;
pub mod music;
pub mod screen;
pub mod signals;
pub mod color;
pub mod playlist;
pub mod styles;
pub mod command_line;
pub mod mode;

use std::sync::mpsc;
use std::{thread, io};
use event::Event;
use termion::input::TermRead;

pub fn init_stdin_thread(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys() {
            if let Ok(key) = key {
                tx.send(Event::Input(key)).unwrap();
            }
        }
    });
}
