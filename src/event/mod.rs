/* Generic representation of something happening in application.
   Used for message passing between objects
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

mod app_event;
mod command_line_event;
mod component_event;
mod nestable_event;
mod screen_event;
mod tagger_event;

pub use app_event::*;
pub use command_line_event::*;
pub use component_event::*;
pub use nestable_event::*;
pub use screen_event::*;
pub use tagger_event::*;

use mpd::Song;
use std::fmt;
use termion::event::Key;

/* Events are sorted into different enums based on their destination
 *
 * App: Goes to and is handled by main application
 * Screen: Goes to current screen
 * Global: Goes to all components
 * Focus: Goes to focused component
 * Mpd: Goes to mpd thread to give instructions to mpd
 */

#[derive(Debug, Clone)]
pub enum Event {
    Dummy,

    BindKey(Vec<Key>, BindableEvent),
    Confirm {
        prompt: String,
        on_yes: Option<ConfirmableEvent>,
        on_no: Option<ConfirmableEvent>,
        is_default_yes: bool,
    },

    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToScreen(ScreenEvent),
    ToComponent(String, ComponentEvent),
    ToFocus(ComponentEvent),
    ToAllComponents(ComponentEvent),
    ToMpd(MpdEvent),
    ToTagger(TaggerEvent),
}

impl Event {
    pub fn err(msg: String) -> Event {
        Event::ToApp(AppEvent::Error(msg))
    }
}

#[derive(Clone)]
pub enum MpdEvent {
    TogglePause,
    Update,
    ClearQueue,
    AddToQueue(Vec<Song>),
    PlayAt(Song),
    Delete(Song),
    Repeat,
    Random,
    Single,
    Consume,
    Next,
    Prev,
    SetVolume(i8),
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
