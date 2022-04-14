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

pub use nestable_event::NestableEvent;

use mpd::Song;
use termion::event::Key;
use std::fmt;

use crate::playlist::Playlist;
use crate::styles::StyleTree;
use crate::mode::Mode;

/* Events are sorted into different enums based on their destination
 *
 * App: Goes to and is handled by main application
 * Screen: Goes to current screen
 * Global: Goes to all components
 * Focus: Goes to focused component
 * Mpd: Goes to mpd thread to give instructions to mpd
 */

#[derive(Debug)]
#[derive(Clone)]
pub enum Event {
    Dummy,

    BindKey(Vec<Key>, NestableEvent),

    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToScreen(ScreenEvent),
    ToComponent(String, ComponentEvent),
    ToFocus(ComponentEvent),
    ToAllComponents(ComponentEvent),
    ToMpd(MpdEvent),
}

#[derive(Clone)]
pub enum ComponentEvent {
    Draw(u16, u16, u16, u16, String),
    Next,
    Prev,
    Select,
    Start,
    GoTo(usize),
    GoToTop,
    GoToBottom,
    Search(String),
    SearchPrev(String),
    NowPlaying(Option<Song>),
    Queue(Vec<Song>),
    Playlist(Vec<Playlist>),
    Database(Vec<Song>),
    PlaylistMenuUpdated(String, Option<Playlist>),
    TagMenuUpdated(String, Vec<usize>),
    StyleMenuUpdated(String, Vec<usize>),
    UpdateRootStyleMenu,
    LostMpdConnection,
}

#[derive(Clone)]
pub enum AppEvent {
    ClearScreen,
    Resize,
    StyleTreeLoaded(Option<StyleTree>),
    SwitchScreen(String),
    Database(Vec<Song>),
    LostMpdConnection,
    DrawScreen,
    Error(String),
    Quit,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CommandLineEvent {
    Echo(String),
    Mode(Mode),
    Input(Key),
    PrevSearch,
    NextSearch,
    SbrcError(usize, String),
    SbrcNotFound,
    MpdOptionChange(mpd::status::Status),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ScreenEvent {
    FocusNext,
    FocusPrev,
    NeedsRedraw(String),
}

#[derive(Clone)]
pub enum MpdEvent {
    TogglePause,
    ClearQueue,
    AddToQueue(Vec<Song>),
    AddStyleToQueue(Vec<String>),
    PlayAt(Song),
    Repeat,
    Random,
    Single,
    Consume,
    Next,
    Prev,
}

impl fmt::Debug for ComponentEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentEvent::NowPlaying(i) =>
                write!(f, "ComponentEvent::NowPlaying({:?})", i),
            ComponentEvent::Queue(s) =>
                write!(f, "ComponentEvent::Queue({} songs)", s.len()),
            ComponentEvent::Playlist(pl) =>
                write!(f, "ComponentEvent::Playlist({} playlists)", pl.len()),
            ComponentEvent::Database(s) =>
                write!(f, "ComponentEvent::Database({} songs)", s.len()),
            ComponentEvent::PlaylistMenuUpdated(t, pl) =>
                write!(f, "ComponentEvent::PlaylistMenuUpdated({}, {} songs)",
                    t,
                    match pl {
                        Some(_) => "Some",
                        None => "None",
                    }
                ),
            ComponentEvent::TagMenuUpdated(t, s) =>
                write!(f, "ComponentEvent::TagMenuUpdated({}, {} songs)",
                    t, s.len()
                ),
            ComponentEvent::UpdateRootStyleMenu =>
                write!(f, "ComponentEvent::UpdateRootStyleMenu"),
            ComponentEvent::StyleMenuUpdated(t, s) =>
                write!(f, "ComponentEvent::StyleMenuUpdated({}, {})",
                    t, s.len()
                ),
            ComponentEvent::LostMpdConnection =>
                write!(f, "ComponentEvent::LostMpdConnection"),
            ComponentEvent::Draw(x, y, w, h, focus) =>
                write!(f, "ComponentEvent::Draw({}, {}, {}, {}, {})",
                    x,
                    y,
                    w,
                    h,
                    focus,
                ),
            ComponentEvent::Next => write!(f, "ComponentEvent::Next"),
            ComponentEvent::Prev => write!(f, "ComponentEvent::Prev"),
            ComponentEvent::Select => write!(f, "ComponentEvent::Select"),
            ComponentEvent::Start => write!(f, "ComponentEvent::Start"),
            ComponentEvent::GoTo(i) => write!(f, "ComponentEvent::GoTo({})", i),
            ComponentEvent::GoToTop => write!(f, "ComponentEvent::GoToTop"),
            ComponentEvent::GoToBottom =>
                write!(f, "ComponentEvent::GoToBottom"),
            ComponentEvent::Search(s) =>
                write!(f, "ComponentEvent::Search({})", s),
            ComponentEvent::SearchPrev(s) =>
                write!(f, "ComponentEvent::SearchPrev({})", s),
        }
    }
}

impl fmt::Debug for MpdEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MpdEvent::TogglePause => write!(f, "MpdEvent::TogglePause"),
            MpdEvent::ClearQueue => write!(f, "MpdEvent::ClearQueue"),
            MpdEvent::AddToQueue(songs) => write!(f, "MpdEvent::AddToQueue({} songs)", songs.len()),
            MpdEvent::AddStyleToQueue(genres) => write!(f, "MpdEvent::AddStyleToQueue({} genres)", genres.len()),
            MpdEvent::PlayAt(song) => write!(f, "MpdEvent::PlayAt({:?})", song),
            MpdEvent::Repeat => write!(f, "MpdEvent::Repeat"),
            MpdEvent::Random => write!(f, "MpdEvent::Random"),
            MpdEvent::Single => write!(f, "MpdEvent::Single"),
            MpdEvent::Consume => write!(f, "MpdEvent::Consume"),
            MpdEvent::Next => write!(f, "MpdEvent::Next"),
            MpdEvent::Prev => write!(f, "MpdEvent::Prev"),
        }
    }
}

impl fmt::Debug for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppEvent::Resize => write!(f, "AppEvent::Resize"),
            AppEvent::Error(s) => write!(f, "AppEvent::Error({:?})", s),
            AppEvent::DrawScreen => write!(f, "AppEvent::DrawScreen"),
            AppEvent::StyleTreeLoaded(_) => write!(f, "AppEvent::StyleTreeLoaded"),
            AppEvent::SwitchScreen(s) => write!(f, "AppEvent::SwitchScreen({:?})", s),
            AppEvent::Database(s) => write!(f, "AppEvent::Database({} songs)", s.len()),
            AppEvent::LostMpdConnection => write!(f, "AppEvent::LostMpdConnection"),
            AppEvent::Quit => write!(f, "AppEvent::Quit"),
            AppEvent::ClearScreen => write!(f, "AppEvent::ClearScreen"),
        }
    }
}
