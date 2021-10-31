use std::sync::mpsc;
use std::thread;

use mpd::idle::{Idle, Subsystem};
use mpd::{Query, Client, Term};

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

            tx.send(spawn_error_msg("Mpd Connection established!")).unwrap();

            if let Some(c) = &mut conn {
                send_database(c, &tx);
                send_queue(c, &tx);
                send_now_playing(c, &tx);
                send_playlists(c, &tx);

                loop {
                    if let Ok(systems) = c.wait(&[]) {
                        for system in systems {
                            match system {
                                Subsystem::Player => send_now_playing(c, &tx),
                                Subsystem::Queue => send_queue(c, &tx),
                                Subsystem::Playlist => send_playlists(c, &tx),
                                Subsystem::Database => send_database(c, &tx),
                                _ => (),
                            }
                        }
                    } else {
                        tx.send(
                            Event::ToApp(AppEvent::LostMpdConnection)
                        ).unwrap();

                        tx.send(spawn_error_msg(
                            "Mpd Connection dropped. Reestablishing \
                                connection..."
                        )).unwrap();

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
        Ok(song) =>
            tx.send(
                Event::ToAllComponents(ComponentEvent::NowPlaying(song))
            ).unwrap(),
        _ =>
            tx.send(
                Event::ToAllComponents(ComponentEvent::NowPlaying(None))
            ).unwrap(),
    }
}

fn send_queue(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.queue() {
        Ok(q) =>
            tx.send(Event::ToAllComponents(ComponentEvent::Queue(q))).unwrap(),
        _ =>
            tx.send(
                Event::ToAllComponents(ComponentEvent::Queue(Vec::new()))
            ).unwrap(),
    }
}

fn send_playlists(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.playlists() {
        Ok(pl) =>
            tx.send(
                Event::ToAllComponents(ComponentEvent::Playlist(
                    pl.iter()
                        .map(|pl| Playlist {
                            name: pl.name.clone(),
                            tracks: match conn.playlist(&pl.name) {
                                Ok(pl) => pl,
                                _ => Vec::new(),
                            },
                        }).collect()
                ))
            ).unwrap(),
        _ =>
            tx.send(
                Event::ToAllComponents(ComponentEvent::Playlist(Vec::new()))
            ).unwrap(),
    }
}

fn send_database(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    let results = conn.search(
        Query::new().and(Term::Any, ""),
        None,
    );

    match results {
        Ok(results) =>
            tx.send(Event::ToApp(AppEvent::Database(results))).unwrap(),
        _ => (),
    }
}
