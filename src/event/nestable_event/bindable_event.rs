/* Version of events that can be bound to a keybind
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
pub enum BindableEvent {
    Confirm {
        prompt: String,
        on_yes: Option<ConfirmableEvent>,
        on_no: Option<ConfirmableEvent>,
        is_default_yes: bool,
    },

    Dummy,

    ToAllComponents(ComponentEvent),
    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToComponent(String, ComponentEvent),
    ToFocus(ComponentEvent),
    ToMpd(MpdEvent),
    ToScreen(ScreenEvent),
}

impl BindableEvent {
    pub fn from_event(e: Event) -> Option<BindableEvent> {
        match e {
            Event::ToApp(e) => Some(BindableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(BindableEvent::ToScreen(e)),
            Event::ToAllComponents(e) => {
                Some(BindableEvent::ToAllComponents(e))
            },
            Event::ToFocus(e) => Some(BindableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(BindableEvent::ToMpd(e)),
            Event::ToCommandLine(e) => Some(BindableEvent::ToCommandLine(e)),
            Event::ToComponent(s, e) => Some(BindableEvent::ToComponent(s, e)),
            Event::Confirm {
                prompt,
                on_yes,
                on_no,
                is_default_yes,
            } => Some(BindableEvent::Confirm {
                prompt,
                on_yes,
                on_no,
                is_default_yes,
            }),
            Event::Dummy => Some(BindableEvent::Dummy),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            BindableEvent::ToApp(e) => Event::ToApp(e),
            BindableEvent::ToScreen(e) => Event::ToScreen(e),
            BindableEvent::ToAllComponents(e) => Event::ToAllComponents(e),
            BindableEvent::ToFocus(e) => Event::ToFocus(e),
            BindableEvent::ToMpd(e) => Event::ToMpd(e),
            BindableEvent::ToCommandLine(e) => Event::ToCommandLine(e),
            BindableEvent::ToComponent(s, e) => Event::ToComponent(s, e),
            BindableEvent::Confirm {
                prompt,
                on_yes,
                on_no,
                is_default_yes,
            } => Event::Confirm {
                prompt,
                on_yes,
                on_no,
                is_default_yes,
            },
            BindableEvent::Dummy => Event::Dummy,
        }
    }
}
