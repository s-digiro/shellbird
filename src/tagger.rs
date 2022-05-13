/* Functionality for tagging files asynchronously
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

use std::fs::{copy, create_dir_all};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use mpd::Song;

use id3::{Tag, TagLike, Version};

use crate::event::{CommandLineEvent, Event, MpdEvent, TaggerEvent};

pub fn init_tagger_thread(
    mut tx: mpsc::Sender<Event>,
) -> mpsc::Sender<TaggerEvent> {
    let (ret_tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut dir = None;
        let mut temp_dir = None;

        loop {
            let event = match rx.recv() {
                Ok(e) => e,
                _ => break, // Main program exited
            };

            match event {
                TaggerEvent::MusicDir(path) => {
                    match string_to_dir_path("MusicDir", &path) {
                        Ok(path) => dir = Some(path),
                        Err(msg) => {
                            dir = None;

                            tx.send(Event::err(msg)).unwrap();
                        },
                    }
                },

                TaggerEvent::TempDir(path) => {
                    match string_to_dir_path("TempDir", &path) {
                        Ok(path) => temp_dir = Some(path),
                        Err(_) => match create_dir_all(&path) {
                            Ok(_) => match string_to_dir_path("TempDir", &path)
                            {
                                Ok(path) => dir = Some(path),
                                Err(msg) => {
                                    dir = None;
                                    tx.send(Event::err(msg)).unwrap();
                                },
                            },
                            Err(e) => {
                                dir = None;
                                tx.send(Event::err(format!(
                                    "Tagger - Create Dir Error: {:?}",
                                    e
                                )))
                                .unwrap();
                            },
                        },
                    }
                },

                TaggerEvent::Tag(songs, tags) => match (&dir, &temp_dir) {
                    (Some(dir), Some(temp_dir)) => {
                        tag(&mut tx, &dir, &temp_dir, songs, tags)
                    },

                    (Some(_), None) => tx
                        .send(Event::err(
                            concat!(
                                "Tagger Error: Cannot tag when MusicDir is ",
                                "not set",
                            )
                            .to_owned(),
                        ))
                        .unwrap(),

                    (None, Some(_)) => tx
                        .send(Event::err(
                            concat!(
                                "Tagger Error: Cannot tag when TempDir is not ",
                                "set"
                            )
                            .to_owned(),
                        ))
                        .unwrap(),

                    (None, None) => tx
                        .send(Event::err(
                            concat!(
                                "Tagger Error: Cannot tag when MusicDir and ",
                                "TempDir are not set",
                            )
                            .to_owned(),
                        ))
                        .unwrap(),
                },
            }
        }
    });

    return ret_tx;
}

fn tag(
    tx: &mut mpsc::Sender<Event>,
    dir: &PathBuf,
    temp_dir: &PathBuf,
    songs: Vec<Song>,
    tags: Vec<(String, Option<String>)>,
) {
    let mut path_pairs = Vec::new();

    for song in songs.iter() {
        let mut temp_path = temp_dir.clone();
        temp_path.push(&song.file);

        let mut path = dir.clone();
        path.push(&song.file);

        if !path.exists() {
            tx.send(Event::err(format!(
                "Tagger: Cannot find file {}",
                path.to_string_lossy()
            )))
            .unwrap();

            return;
        }

        if let Err(e) = copy(&path, &temp_path) {
            match e.kind() {
                io::ErrorKind::NotFound => match temp_path.as_path().parent() {
                    Some(temp_dir) => {
                        if let Err(e) = create_dir_all(&temp_dir) {
                            tx.send(Event::err(format!(
                                "Tagger - Create Dir Error: {:?}",
                                e
                            )))
                            .unwrap();

                            return;
                        }

                        if let Err(e) = copy(&path, &temp_path) {
                            tx.send(Event::err(format!(
                                "Tagger - Copy Error: {:?}",
                                e
                            )))
                            .unwrap();

                            return;
                        }
                    },
                    None => {
                        tx.send(Event::err(format!(
                            "Tagger - Copy Error: {:?}",
                            e
                        )))
                        .unwrap();

                        return;
                    },
                },
                _ => {
                    tx.send(Event::err(format!(
                        "Tagger - Copy Error: {:?}",
                        e
                    )))
                    .unwrap();

                    return;
                },
            }
        }

        let mut tag = match Tag::read_from_path(&temp_path) {
            Ok(tag) => tag,
            Err(id3::Error {
                kind: id3::ErrorKind::NoTag,
                ..
            }) => Tag::new(),
            Err(e) => {
                tx.send(Event::err(format!("Tagger - Id3 Tag Error: {:?}", e)))
                    .unwrap();

                return;
            },
        };

        for (key, val) in tags.iter() {
            match key.as_str() {
                "Title" => match val {
                    Some(val) => tag.set_title(val),
                    None => tag.remove_title(),
                },
                "Artist" => match val {
                    Some(val) => tag.set_artist(val),
                    None => tag.remove_artist(),
                },
                "AlbumArtist" => match val {
                    Some(val) => tag.set_album_artist(val),
                    None => tag.remove_album_artist(),
                },
                "Album" => match val {
                    Some(val) => tag.set_album(val),
                    None => tag.remove_album(),
                },
                "Date" => match val {
                    Some(val) => {
                        tag.set_text("TYER", val);
                        tag.set_text("TDRC", val);
                    },
                    None => {
                        tag.remove("TYER");
                        tag.remove("TDRC");
                    },
                },
                "Track" => match val {
                    Some(val) => tag.set_text("TRCK", val),
                    None => {
                        tag.remove("TRCK");
                    },
                },
                "Genre" => match val {
                    Some(val) => tag.set_genre(val),
                    None => tag.remove_genre(),
                },
                "Composer" => match val {
                    Some(val) => tag.set_text("TCOM", val),
                    None => {
                        tag.remove("TCOM");
                    },
                },
                "Disc" => match val {
                    Some(val) => tag.set_text("TPOS", val),
                    None => {
                        tag.remove("TPOS");
                    },
                },
                other => {
                    tx.send(Event::err(format!(
                        "Tagger Error: Cannot tag unhandled tag key '{}'",
                        other
                    )))
                    .unwrap();

                    return;
                },
            }
        }

        if let Err(e) = tag.write_to_path(&temp_path, Version::Id3v24) {
            tx.send(Event::err(format!("Tagger - Write Error: {:?}", e)))
                .unwrap();

            return;
        }

        path_pairs.push((temp_path, path));
    }

    for (temp_path, path) in path_pairs {
        if let Err(e) = copy(&temp_path, &path) {
            tx.send(Event::err(format!("Tagger - Copy Error: {:?}", e)))
                .unwrap();
        }
    }

    tx.send(Event::ToCommandLine(CommandLineEvent::Echo(
        "Saved tags!".to_owned(),
    )))
    .unwrap();
    tx.send(Event::ToMpd(MpdEvent::Update)).unwrap();
}

fn string_to_dir_path(name: &str, path: &str) -> Result<PathBuf, String> {
    let ret = PathBuf::from(shellexpand::tilde(path).into_owned());

    if !ret.is_dir() {
        let msg = format!(
            concat!(
                "Cannot set {} for tagging to '{}' because it is not a valid ",
                "folder."
            ),
            name,
            ret.to_string_lossy(),
        );

        return Err(msg);
    }

    Ok(ret)
}
