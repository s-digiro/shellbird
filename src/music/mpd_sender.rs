/* Functionality for sending data to mpd
   Copyright (C) 2020-2022 Sean DiGirolamo

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

use std::io::ErrorKind as IoErrorKind;
use std::sync::mpsc;
use std::thread;

use crate::event::*;

use mpd::error::{Error, ParseError};
use mpd::{Client, Song};

pub fn init_mpd_sender_thread(
    ip: &str,
    port: &str,
    tx: mpsc::Sender<Event>,
) -> mpsc::Sender<MpdEvent> {
    let (ret_tx, rx) = mpsc::channel();

    let ip = ip.to_string();
    let port = port.to_string();

    let rethrow_tx: mpsc::Sender<MpdEvent> = ret_tx.clone();
    thread::spawn(move || {
        let mut conn = None;

        loop {
            while let None = conn {
                conn = super::get_mpd_conn(&ip, &port);
            }

            if let Some(c) = &mut conn {
                let request = match rx.recv() {
                    Ok(command) => command,
                    _ => break, // Main program exited
                };

                let result = match request.clone() {
                    MpdEvent::TogglePause => c.toggle_pause(),
                    MpdEvent::Update => match c.update() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    },
                    MpdEvent::Repeat => toggle_repeat(c),
                    MpdEvent::Random => toggle_random(c),
                    MpdEvent::Single => toggle_single(c),
                    MpdEvent::Consume => toggle_consume(c),
                    MpdEvent::ClearQueue => c.clear(),
                    MpdEvent::AddToQueue(songs) => push_all(c, songs),
                    MpdEvent::PlayAt(song) => play_at(c, song),
                    MpdEvent::Delete(song) => delete(c, song),
                    MpdEvent::Next => c.next(),
                    MpdEvent::Prev => c.prev(),
                    MpdEvent::SetVolume(vol) => c.volume(vol),
                };

                if let Err(e) = result {
                    let retry = match &e {
                        Error::Parse(ParseError::BadPair) => true,
                        Error::Io(e) => match e.kind() {
                            IoErrorKind::BrokenPipe => true,
                            _ => false,
                        },
                        _ => false,
                    };

                    if retry {
                        // Connection dropped, reestablish and retry
                        rethrow_tx.send(request).unwrap();
                    } else {
                        tx.send(Event::ToApp(AppEvent::Error(format!(
                            "{:?}",
                            e
                        ))))
                        .unwrap();
                    }

                    conn = None;
                }
            }
        }
    });

    ret_tx
}

fn toggle_repeat(conn: &mut Client) -> Result<(), Error> {
    let status = conn.status()?;
    conn.repeat(!status.repeat)
}

fn toggle_random(conn: &mut Client) -> Result<(), Error> {
    let status = conn.status()?;
    conn.random(!status.random)
}

fn toggle_single(conn: &mut Client) -> Result<(), Error> {
    let status = conn.status()?;
    conn.single(!status.single)
}

fn toggle_consume(conn: &mut Client) -> Result<(), Error> {
    let status = conn.status()?;
    conn.consume(!status.consume)
}

fn push_all(conn: &mut Client, songs: Vec<Song>) -> Result<(), Error> {
    for song in songs {
        conn.push(song)?;
    }

    Ok(())
}

fn play_at(conn: &mut Client, song: Song) -> Result<(), Error> {
    match song.place {
        Some(place) => conn.switch(place.pos),
        None => {
            conn.push(song)?;
            let q = conn.queue()?;
            conn.switch(q.last().unwrap().place.unwrap().pos).unwrap();

            Ok(())
        },
    }
}

fn delete(conn: &mut Client, song: Song) -> Result<(), Error> {
    match song.place {
        Some(place) => conn.delete(place.pos),
        None => Ok(()),
    }
}
