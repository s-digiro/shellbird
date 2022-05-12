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

mod nestable_event;

pub use nestable_event::*;

use mpd::Song;
use std::fmt;
use termion::event::Key;

use crate::color::Color;
use crate::playlist::Playlist;
use crate::styles::StyleTree;

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

#[derive(Debug, Clone)]
pub enum TaggerEvent {
    MusicDir(String),
    TempDir(String),
    Tag(Vec<Song>, Vec<(String, Option<String>)>),
}

#[derive(Clone)]
pub enum ComponentEvent {
    ReturnText(String),
    Draw(u16, u16, u16, u16, String),
    OpenTags,
    Next,
    Prev,
    Select,
    Delete,
    Start,
    GoTo(usize),
    GoToTop,
    GoToBottom,
    Search(String),
    SearchPrev(String),
    NowPlaying(Option<usize>),
    Queue(Vec<usize>),
    Playlist(Vec<Playlist>),
    Database,
    PlaylistMenuUpdated(String, Vec<usize>),
    TagMenuUpdated(String, Vec<usize>),
    StyleMenuUpdated(String, Vec<usize>),
    UpdateRootStyleMenu,
    LostMpdConnection,
}

#[derive(Clone)]
pub enum AppEvent {
    TagUI(Vec<usize>),
    NowPlaying(Option<Song>),
    Back,
    ClearScreen,
    Resize,
    StyleTreeLoaded(Option<StyleTree>),
    SwitchScreen(String),
    Database(Vec<Song>),
    Queue(Vec<Song>),
    LostMpdConnection,
    DrawScreen,
    Error(String),
    Quit,
}

#[derive(Debug, Clone)]
pub enum CommandLineEvent {
    Echo(String),
    RequestText(String, Option<String>),
    SetColor(Color),
    Input(Key),
    PrevSearch,
    NextSearch,
    SbrcError(usize, String),
    SbrcNotFound,
    MpdStatus(mpd::status::Status),
    VolumeMv(i8),
    VolumeUp(i8),
    VolumeDown(i8),
}

#[derive(Debug, Clone)]
pub enum ScreenEvent {
    FocusNext,
    FocusPrev,
    NeedsRedraw(String),
}

#[derive(Clone)]
pub enum MpdEvent {
    TogglePause,
    Update,
    ClearQueue,
    AddToQueue(Vec<Song>),
    AddStyleToQueue(Vec<String>),
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

impl fmt::Debug for ComponentEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentEvent::NowPlaying(i) => {
                write!(f, "ComponentEvent::NowPlaying({:?})", i)
            },
            ComponentEvent::ReturnText(prompt) => {
                write!(f, "ComponentEvent::ReturnText({:?})", prompt)
            },
            ComponentEvent::Queue(s) => {
                write!(f, "ComponentEvent::Queue({} ids)", s.len())
            },
            ComponentEvent::Playlist(pl) => {
                write!(f, "ComponentEvent::Playlist({} playlists)", pl.len())
            },
            ComponentEvent::Database => {
                write!(f, "ComponentEvent::Database")
            },
            ComponentEvent::PlaylistMenuUpdated(t, pl) => write!(
                f,
                "ComponentEvent::PlaylistMenuUpdated({}, {} songs)",
                t,
                pl.len()
            ),
            ComponentEvent::TagMenuUpdated(t, s) => write!(
                f,
                "ComponentEvent::TagMenuUpdated({}, {} songs)",
                t,
                s.len()
            ),
            ComponentEvent::UpdateRootStyleMenu => {
                write!(f, "ComponentEvent::UpdateRootStyleMenu")
            },
            ComponentEvent::StyleMenuUpdated(t, s) => {
                write!(
                    f,
                    "ComponentEvent::StyleMenuUpdated({}, {})",
                    t,
                    s.len()
                )
            },
            ComponentEvent::LostMpdConnection => {
                write!(f, "ComponentEvent::LostMpdConnection")
            },
            ComponentEvent::Delete => write!(f, "ComponentEvent::Delete"),
            ComponentEvent::Draw(x, y, w, h, focus) => write!(
                f,
                "ComponentEvent::Draw({}, {}, {}, {}, {})",
                x, y, w, h, focus,
            ),
            ComponentEvent::Next => write!(f, "ComponentEvent::Next"),
            ComponentEvent::OpenTags => write!(f, "ComponentEvent::OpenTags"),
            ComponentEvent::Prev => write!(f, "ComponentEvent::Prev"),
            ComponentEvent::Select => write!(f, "ComponentEvent::Select"),
            ComponentEvent::Start => write!(f, "ComponentEvent::Start"),
            ComponentEvent::GoTo(i) => write!(f, "ComponentEvent::GoTo({})", i),
            ComponentEvent::GoToTop => write!(f, "ComponentEvent::GoToTop"),
            ComponentEvent::GoToBottom => {
                write!(f, "ComponentEvent::GoToBottom")
            },
            ComponentEvent::Search(s) => {
                write!(f, "ComponentEvent::Search({})", s)
            },
            ComponentEvent::SearchPrev(s) => {
                write!(f, "ComponentEvent::SearchPrev({})", s)
            },
        }
    }
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
            MpdEvent::AddStyleToQueue(genres) => {
                write!(f, "MpdEvent::AddStyleToQueue({} genres)", genres.len())
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

impl fmt::Debug for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppEvent::Resize => write!(f, "AppEvent::Resize"),
            AppEvent::NowPlaying(song) => write!(f, "AppEvent::NowPlaying({:?})", song),
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
            AppEvent::Queue(s) => write!(f, "AppEvent::Queue({} songs)", s.len()),
        }
    }
}
