/* Events sent to and handled by MPD Sender Thread
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

use mpd::Song;
use std::fmt;

#[derive(Clone)]
pub enum MpdEvent {
    // Instruct MPD to add songs in Vec to track queue
    AddToQueue(Vec<Song>),

    // Instruct MPD to clear the track queue
    ClearQueue,

    // Instructs MPD to toggle Consume mode
    Consume,

    // Instructs MPD to delete song from queue
    Delete(Song),

    // Instructs MPD to skip to next track
    Next,

    // Instructs MPD to play at song in queue
    PlayAt(Song),

    // Instructs MPD to skip to previous track
    Prev,

    // Instructs MPD to toggle Random mode
    Random,

    // Instructs MPD to toggle Repeat mode
    Repeat,

    // Instructs MPD to set volume to i8
    SetVolume(i8),

    // Instructs MPD to toggle Single mode
    Single,

    // Instruct MPD to toggle MPD state between playing and paused
    TogglePause,

    // Instruct MPD to perform a database updated
    Update,
}

impl fmt::Debug for MpdEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MpdEvent::TogglePause => write!(f, "MpdEvent::TogglePause"),
            MpdEvent::Update => write!(f, "MpdEvent::Update"),
            MpdEvent::ClearQueue => write!(f, "MpdEvent::ClearQueue"),
            MpdEvent::AddToQueue(songs) => {
                write!(f, "MpdEvent::AddToQueue({} songs)", songs.len())
            },
            MpdEvent::Delete(song) => write!(f, "MpdEvent::Delete({:?})", song),
            MpdEvent::PlayAt(song) => write!(f, "MpdEvent::PlayAt({:?})", song),
            MpdEvent::Repeat => write!(f, "MpdEvent::Repeat"),
            MpdEvent::Random => write!(f, "MpdEvent::Random"),
            MpdEvent::Single => write!(f, "MpdEvent::Single"),
            MpdEvent::Consume => write!(f, "MpdEvent::Consume"),
            MpdEvent::Next => write!(f, "MpdEvent::Next"),
            MpdEvent::Prev => write!(f, "MpdEvent::Prev"),
            MpdEvent::SetVolume(vol) => {
                write!(f, "MpdEvent::SetVolume({})", vol)
            },
        }
    }
}
