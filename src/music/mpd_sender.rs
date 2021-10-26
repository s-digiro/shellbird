use std::sync::mpsc;
use std::borrow::Cow;
use std::thread;

use crate::event::*;

use mpd::Query;
use mpd::Term;
use mpd::Client;
use mpd::error::Error;
use mpd::Song;

pub fn init_mpd_sender_thread(
    ip: &str,
    port: &str,
    tx: mpsc::Sender<Event>
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
                    MpdEvent::Random => toggle_random(c),
                    MpdEvent::ClearQueue => c.clear(),
                    MpdEvent::AddToQueue(songs) => push_all(c, songs),
                    MpdEvent::PlayAt(song) => play_at(c, song),
                    MpdEvent::AddStyleToQueue(genres) => add_style_to_queue(c, genres),
                };

                if let Err(_) = result {
                    tx.send(Event::ToApp(AppEvent::Error(format!(
                        "Mpd Sender Thread: Mpd Connection dropped. Resending \
                            MpdRequest {:?}",
                        request
                    )))).unwrap();
                    conn = None;
                    rethrow_tx.send(request).unwrap();
                }
            }
        }
    });

    ret_tx
}

fn toggle_random(conn: &mut Client) -> Result<(), Error> {
    let stats = conn.status()?;

    conn.random(!stats.random)?;

    Ok(())
}

fn push_all(conn: &mut Client, songs: Vec<Song>) -> Result<(), Error> {
    for song in songs {
        if let Err(e) = conn.push(song) {
            return Err(e)
        }
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

fn add_style_to_queue(conn: &mut Client, genres: Vec<String>) -> Result<(), Error> {
    for genre in genres {
        let songs = conn.search(
            Query::new()
                .and(
                    Term::Tag(Cow::Borrowed("Genre")),
                    genre
                ),
                None
        )?;

        push_all(conn, songs)?;
    }

    Ok(())
}
