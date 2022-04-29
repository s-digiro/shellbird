/* Functionality related to getting data from mpd
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

use mpd::idle::{Idle, Subsystem};
use mpd::{Client, Query, Term};

use super::*;
use crate::event::*;
use crate::playlist::Playlist;

pub fn init_mpd_listener_thread(ip: &str, port: &str, tx: mpsc::Sender<Event>) {
    let ip = ip.to_string();
    let port = port.to_string();

    thread::spawn(move || {
        let mut conn = None;

        loop {
            while let None = conn {
                conn = get_mpd_conn(&ip, &port);
            }

            tx.send(spawn_error_msg("Mpd Connection established!"))
                .unwrap();

            if let Some(c) = &mut conn {
                send_status(c, &tx);
                send_database(c, &tx);
                send_now_playing(c, &tx);
                send_queue(c, &tx);
                send_playlists(c, &tx);

                loop {
                    if let Ok(systems) = c.wait(&[]) {
                        for system in systems {
                            match system {
                                Subsystem::Player => send_now_playing(c, &tx),
                                Subsystem::Queue => send_queue(c, &tx),
                                Subsystem::Playlist => send_playlists(c, &tx),
                                Subsystem::Database => send_database(c, &tx),
                                Subsystem::Options => send_status(c, &tx),
                                Subsystem::Mixer => send_status(c, &tx),
                                _ => (),
                            }
                        }
                    } else {
                        tx.send(Event::ToApp(AppEvent::LostMpdConnection)).unwrap();

                        tx.send(spawn_error_msg(
                            "Mpd Connection dropped. Reestablishing connection...",
                        ))
                        .unwrap();

                        conn = None;
                        break;
                    }
                }
            }
        }
    });
}

fn spawn_error_msg(msg: &str) -> Event {
    let msg = format!("Mpd Listener Thread: {}", msg);
    Event::ToApp(AppEvent::Error(msg))
}

fn send_now_playing(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.currentsong() {
        Ok(song) => tx
            .send(Event::ToAllComponents(ComponentEvent::NowPlaying(song)))
            .unwrap(),
        _ => tx
            .send(Event::ToAllComponents(ComponentEvent::NowPlaying(None)))
            .unwrap(),
    }
}

fn send_queue(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.queue() {
        Ok(q) => tx
            .send(Event::ToAllComponents(ComponentEvent::Queue(q)))
            .unwrap(),
        _ => tx
            .send(Event::ToAllComponents(ComponentEvent::Queue(Vec::new())))
            .unwrap(),
    }
}

fn send_playlists(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.playlists() {
        Ok(pl) => tx
            .send(Event::ToAllComponents(ComponentEvent::Playlist(
                pl.iter()
                    .map(|pl| Playlist {
                        name: pl.name.clone(),
                        tracks: match conn.playlist(&pl.name) {
                            Ok(pl) => pl,
                            _ => Vec::new(),
                        },
                    })
                    .collect(),
            )))
            .unwrap(),
        _ => tx
            .send(Event::ToAllComponents(ComponentEvent::Playlist(Vec::new())))
            .unwrap(),
    }
}

fn send_database(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    let results = conn.search(Query::new().and(Term::Any, ""), None);

    match results {
        Ok(results) => tx.send(Event::ToApp(AppEvent::Database(results))).unwrap(),
        _ => (),
    }
}

fn send_status(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.status() {
        Ok(status) => tx
            .send(Event::ToCommandLine(CommandLineEvent::MpdStatus(status)))
            .unwrap(),
        err => eprintln!("{:?}", err),
    }
}
