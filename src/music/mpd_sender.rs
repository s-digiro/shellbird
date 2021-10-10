use std::sync::mpsc;
use std::borrow::Cow;
use std::thread;

use crate::event::*;
use crate::music;

use mpd::Query;
use mpd::Term;
use mpd::Client;

pub fn init_mpd_sender_thread(ip: &str, port: &str) -> mpsc::Sender<MpdEvent> {
    let (tx, rx) = mpsc::channel();

    let ip = ip.to_string();
    let port = port.to_string();

    thread::spawn(move || {
        let mut conn = music::get_mpd_conn(&ip, &port).unwrap();

        loop {
            let request = match rx.recv() {
                Ok(command) => command,
                _ => break, // Main program exited
            };

            while let Err(_) = conn.ping() {
                if let Some(c) = music::get_mpd_conn(&ip, &port) {
                    conn = c;
                }
            }

            match request {
                MpdEvent::TogglePause => conn.toggle_pause().unwrap(),
                MpdEvent::Random => toggle_random(&mut conn),
                MpdEvent::ClearQueue => conn.clear().unwrap(),
                MpdEvent::AddToQueue(songs) => for song in songs {
                    conn.push(song).unwrap();
                },
                MpdEvent::PlayAt(song) => match song.place {
                    Some(place) => conn.switch(place.pos).unwrap(),
                    None => {
                        conn.push(song).unwrap();
                        let q = conn.queue().unwrap();
                        conn.switch(q.last().unwrap().place.unwrap().pos).unwrap();
                    },
                },
                MpdEvent::AddStyleToQueue(genres) => {
                    for genre in genres {
                        if let Ok(songs) = conn.search(
                            Query::new()
                                .and(
                                    Term::Tag(Cow::Borrowed("Genre")),
                                    genre
                                ),
                                None
                        ) {
                            for song in songs {
                                conn.push(song).unwrap();
                            }
                        }
                    }
                }
            }
        }
    });

    tx
}

fn toggle_random(conn: &mut Client) {
    let stats = conn.status().unwrap();

    conn.random(!stats.random).unwrap();
}
