/* Version of events that can be passed in confirm event
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

/* TODO: Convert weird conversion methods into into implementations */

use super::*;

#[derive(Debug, Clone)]
pub enum ConfirmableEvent {
    Dummy,

    ToAllComponents(ComponentEvent),
    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToComponent(String, ComponentEvent),
    ToFocus(ComponentEvent),
    ToMpd(MpdEvent),
    ToScreen(ScreenEvent),
}

impl ConfirmableEvent {
    pub fn from_event(e: Event) -> Option<ConfirmableEvent> {
        match e {
            Event::ToApp(e) => Some(ConfirmableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(ConfirmableEvent::ToScreen(e)),
            Event::ToAllComponents(e) => {
                Some(ConfirmableEvent::ToAllComponents(e))
            },
            Event::ToFocus(e) => Some(ConfirmableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(ConfirmableEvent::ToMpd(e)),
            Event::ToCommandLine(e) => Some(ConfirmableEvent::ToCommandLine(e)),
            Event::ToComponent(s, e) => {
                Some(ConfirmableEvent::ToComponent(s, e))
            },
            Event::Dummy => Some(ConfirmableEvent::Dummy),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            ConfirmableEvent::ToApp(e) => Event::ToApp(e),
            ConfirmableEvent::ToScreen(e) => Event::ToScreen(e),
            ConfirmableEvent::ToAllComponents(e) => Event::ToAllComponents(e),
            ConfirmableEvent::ToFocus(e) => Event::ToFocus(e),
            ConfirmableEvent::ToMpd(e) => Event::ToMpd(e),
            ConfirmableEvent::ToCommandLine(e) => Event::ToCommandLine(e),
            ConfirmableEvent::ToComponent(s, e) => Event::ToComponent(s, e),
            ConfirmableEvent::Dummy => Event::Dummy,
        }
    }
}
