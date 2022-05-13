/* Events sent to and handled by any struct with Component Trait
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

use std::fmt;

use crate::playlist::Playlist;

/* TODO: There seems to be two categories of events. Events that are sent by
 * the user, and events that are sent by the application. Maybe there is a
 * way to better separate them?
 *
 * Maybe usize should be a type SongId instead? */

#[derive(Clone)]
pub enum ComponentEvent {
    // Instructs Component to update internal state to reflect a change in the
    // mpd library database
    Database,

    // Instructs Component to 'delete'. Like clicking delete if it existed on
    // a gamecube??? Behavior differs between individual components
    Delete,

    // Instructs component to draw itself on the screen
    Draw(u16, u16, u16, u16, String),

    // Instructs menu component to move selection to a given line.
    GoTo(usize),

    // Instructs menu component to move selection to last line
    GoToBottom,

    // Instructs menu component to move selection to first line
    GoToTop,

    // Informs a component that we lost our MPD connection and that it should
    // update its internal state
    LostMpdConnection,

    // Instructs component to Open the tag editor. Behavior differs between
    // individual components
    OpenTags,

    // Instructs Component to move to next item. Behavior differs between
    // individual components
    Next,

    // Instructs component to update internal state to reflect a new currently
    // playing track
    NowPlaying(Option<usize>),

    // Instructs Component to update internal state to reflect a change in a
    // playlist
    Playlist(Vec<Playlist>),

    // Informs a component that another component with name String of type
    // PlaylistMenu has changed its selection to list of song ids Vec<usize>.
    // If this component has that component as a parent, it should update its
    // internal state
    PlaylistMenuUpdated(String, Vec<usize>),

    // Instructs Component to move to previous item. Behavior differs between
    // individual components
    Prev,

    // Instructs Component to update internal state to reflect a change in the
    // MPD track queue
    Queue(Vec<usize>),

    // Return text to a component (Used exclusively by CommandLine after a
    // Component Requests text from it)
    ReturnText(String),

    // Searches for next occurance of a string in a menu component and moves
    // selection to that line. Case insensitive
    Search(String),

    // Searches for previous occurance of a string in a menu component and
    // moves selection to that line. Case insensitive
    SearchPrev(String),

    // Instructs Component to 'select'. Like clicking "A" on a gamecube.
    // Behavior differs between individual components
    Select,

    // Instructs Component to 'start'. Like clicking "Start" on a gamecube.
    // Behavior differs between individual components. A lot of times it does
    // the same thing as Select
    Start,

    // Informs a component that another component with name String of type
    // StyleMenu has changed its selection to list of song ids Vec<usize>. If
    // this component has that component as a parent, it should update its
    // internal state
    StyleMenuUpdated(String, Vec<usize>),

    // Informs a component that another component with name String of type
    // TagMenu has changed its selection to list of song ids Vec<usize>. If
    // this component has that component as a parent, it should update its
    // internal state
    TagMenuUpdated(String, Vec<usize>),

    // Informs a component that the loaded style tree has changes. If this
    // component is a style menu with no parent, it should update its internal
    // state
    UpdateRootStyleMenu,
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
