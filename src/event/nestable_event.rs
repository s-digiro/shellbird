/* Events that can go in other events
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

use super::*;

#[derive(Debug, Clone)]
pub enum NestableEvent {
    ToApp(AppEvent),
    ToScreen(ScreenEvent),
    ToAllComponents(ComponentEvent),
    ToFocus(ComponentEvent),
    ToMpd(MpdEvent),
    ToCommandLine(CommandLineEvent),
    ToComponent(String, ComponentEvent),
}

impl NestableEvent {
    pub fn from_event(e: Event) -> Option<NestableEvent> {
        match e {
            Event::ToApp(e) => Some(NestableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(NestableEvent::ToScreen(e)),
            Event::ToAllComponents(e) => Some(NestableEvent::ToAllComponents(e)),
            Event::ToFocus(e) => Some(NestableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(NestableEvent::ToMpd(e)),
            Event::ToCommandLine(e) => Some(NestableEvent::ToCommandLine(e)),
            Event::ToComponent(s, e) => Some(NestableEvent::ToComponent(s, e)),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            NestableEvent::ToApp(e) => Event::ToApp(e),
            NestableEvent::ToScreen(e) => Event::ToScreen(e),
            NestableEvent::ToAllComponents(e) => Event::ToAllComponents(e),
            NestableEvent::ToFocus(e) => Event::ToFocus(e),
            NestableEvent::ToMpd(e) => Event::ToMpd(e),
            NestableEvent::ToCommandLine(e) => Event::ToCommandLine(e),
            NestableEvent::ToComponent(s, e) => Event::ToComponent(s, e),
        }
    }
}
