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
mod mpd_event;
mod nestable_event;
mod screen_event;
mod tagger_event;

pub use app_event::*;
pub use command_line_event::*;
pub use component_event::*;
pub use mpd_event::*;
pub use nestable_event::*;
pub use screen_event::*;
pub use tagger_event::*;

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
    // Bind sequence of keys to an event
    BindKey(Vec<Key>, BindableEvent),

    // Prompt a user for y/n and triggers another event based on input
    Confirm {
        prompt: String,
        on_yes: Option<ConfirmableEvent>,
        on_no: Option<ConfirmableEvent>,
        is_default_yes: bool,
    },

    // Does Nothing
    Dummy,

    ToAllComponents(ComponentEvent),
    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToComponent(String, ComponentEvent),
    ToFocus(ComponentEvent),
    ToMpd(MpdEvent),
    ToScreen(ScreenEvent),
    ToTagger(TaggerEvent),
}

impl Event {
    pub fn err(msg: String) -> Event {
        Event::ToApp(AppEvent::Error(msg))
    }
}
