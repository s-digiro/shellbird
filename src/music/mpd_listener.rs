use std::sync::mpsc;
use std::thread;

use mpd::idle::{Idle, Subsystem};
use mpd::{Query, Client, Term};

use super::*;
use crate::event::*;
use crate::playlist::Playlist;

pub fn init_mpd_listener_thread(ip: &str, port: &str, tx: mpsc::Sender<Event>) {
    let mut conn = get_mpd_conn(ip, port).unwrap();

    thread::spawn(move || {

        send_database(&mut conn, &tx);
        send_queue(&mut conn, &tx);
        send_now_playing(&mut conn, &tx);
        send_playlists(&mut conn, &tx);

        loop {
            if let Ok(systems) = conn.wait(&[]) {
                for system in systems {
                    match system {
                        Subsystem::Player => send_now_playing(&mut conn, &tx),
                        Subsystem::Queue => send_queue(&mut conn, &tx),
                        Subsystem::Playlist => send_playlists(&mut conn, &tx),
                        Subsystem::Database => send_database(&mut conn, &tx),
                        _ => (),
                    }
                }
            }
        }
    });
}

fn send_now_playing(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.currentsong() {
        Ok(song) => tx.send(Event::ToGlobal(GlobalEvent::NowPlaying(song))).unwrap(),
        _ => tx.send(Event::ToGlobal(GlobalEvent::NowPlaying(None))).unwrap(),
    }
}

fn send_queue(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.queue() {
        Ok(q) => tx.send(Event::ToGlobal(GlobalEvent::Queue(q))).unwrap(),
        _ => tx.send(Event::ToGlobal(GlobalEvent::Queue(Vec::new()))).unwrap(),
    }
}

fn send_playlists(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    match conn.playlists() {
        Ok(pl) => tx.send(
            Event::ToGlobal(GlobalEvent::Playlist(
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
        _ => tx.send(Event::ToGlobal(GlobalEvent::Playlist(Vec::new()))).unwrap(),
    }
}

fn send_database(conn: &mut Client, tx: &mpsc::Sender<Event>) {
    let results = conn.search(
        Query::new().and(Term::Any, ""),
        None,
    );

    match results {
        Ok(results) => tx.send(Event::ToGlobal(GlobalEvent::Database(results))).unwrap(),
        _ => (),
    }
}
