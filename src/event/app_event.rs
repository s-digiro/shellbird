/* Events sent to and handled by main application loop
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

use crate::StyleTree;
use mpd::Song;
use std::fmt;

#[derive(Clone)]
pub enum AppEvent {
    // Instruct App to exit TagUI and return to previous screen
    Back,

    // Clear terminal screen
    ClearScreen,

    // Inform App that MPD library database has changed. Also sends library.
    // Sent by MPD Listener Thread
    Database(Vec<Song>),

    // Instructs App to draw the current screen to terminal
    DrawScreen,

    // Informs App that an error has occurred. It will also be drawn in
    // statusbar
    Error(String),

    // Inform App that MPD Connection has been lost
    LostMpdConnection,

    // Inform app that Currently playing track has changes. Sent from Mpd
    // Sender Thread
    NowPlaying(Option<Song>),

    // Inform App that Mpd Queue has changed
    Queue(Vec<Song>),

    // Exit App
    Quit,

    // Inform App that terminal window has been resized
    Resize,

    // Inform App that Style tree has finished loading
    StyleTreeLoaded(Option<StyleTree>),

    // Switch screen to String
    SwitchScreen(String),

    // Open TagUI on songs with Ids in Vec
    TagUI(Vec<usize>),
}

impl fmt::Debug for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppEvent::Resize => write!(f, "AppEvent::Resize"),
            AppEvent::NowPlaying(song) => {
                write!(f, "AppEvent::NowPlaying({:?})", song)
            },
            AppEvent::Back => write!(f, "AppEvent::Back"),
            AppEvent::TagUI(songs) => {
                write!(f, "AppEvent::TagUI({} songs)", songs.len())
            },
            AppEvent::Error(s) => write!(f, "AppEvent::Error({:?})", s),
            AppEvent::DrawScreen => write!(f, "AppEvent::DrawScreen"),
            AppEvent::StyleTreeLoaded(_) => {
                write!(f, "AppEvent::StyleTreeLoaded")
            },
            AppEvent::SwitchScreen(s) => {
                write!(f, "AppEvent::SwitchScreen({:?})", s)
            },
            AppEvent::Database(s) => {
                write!(f, "AppEvent::Database({} songs)", s.len())
            },
            AppEvent::LostMpdConnection => {
                write!(f, "AppEvent::LostMpdConnection")
            },
            AppEvent::Quit => write!(f, "AppEvent::Quit"),
            AppEvent::ClearScreen => write!(f, "AppEvent::ClearScreen"),
            AppEvent::Queue(s) => {
                write!(f, "AppEvent::Queue({} songs)", s.len())
            },
        }
    }
}
