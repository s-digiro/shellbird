use std::sync::mpsc;
use std::thread;

use signal_hook::{
    consts::{
        SIGWINCH,
        SIGINT,
    },
    iterator::Signals,
    low_level
};

use crate::event::*;

pub fn init_listener(tx: mpsc::Sender<Event>) {
    let mut signals = Signals::new(&[SIGWINCH, SIGINT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGWINCH => {
                    tx.send(Event::ToApp(AppEvent::Resize)).unwrap();
                },
                _ => low_level::emulate_default_handler(sig).unwrap(),
            }
        }
    });
}
