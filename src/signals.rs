/* Contains application signal functionality
   Copyright (C) 2020-2021 Sean DiGirolamo

This file is part of Shellbird.

Shellbird is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation; either version 3, or (at your option) any
later version.

Shellbird is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Shellbird; see the file COPYING.  If not see
<http://www.gnu.org/licenses/>.  */

use std::sync::mpsc;
use std::thread;

use signal_hook::{
    consts::{SIGINT, SIGWINCH},
    iterator::Signals,
    low_level,
};

use crate::event::*;

pub fn init_listener(tx: mpsc::Sender<Event>) {
    let mut signals = Signals::new(&[SIGWINCH, SIGINT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGWINCH => {
                    tx.send(Event::ToApp(AppEvent::Resize)).unwrap();
                }
                _ => low_level::emulate_default_handler(sig).unwrap(),
            }
        }
    });
}
