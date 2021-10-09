use std::sync::mpsc;
use std::thread;

use signal_hook::{consts::SIGWINCH, iterator::Signals, low_level};

use crate::Event;

pub fn init_listener(tx: mpsc::Sender<Event>) {
    let mut signals = Signals::new(&[SIGWINCH]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                signal_hook::consts::SIGWINCH => {
                    tx.send(Event::Resize).unwrap();
                },
                _ => low_level::emulate_default_handler(sig).unwrap(),
            }
        }
    });
}
