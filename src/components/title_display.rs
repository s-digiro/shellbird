/* TUI Component for displaying title of currently playing track
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

use crate::color::Color;
use crate::components::{Component, Components};
use crate::event::*;
use crate::GlobalState;
use std::sync::mpsc;
use termion::{color, cursor};
use unicode_truncate::{Alignment, UnicodeTruncateStr};

#[derive(Debug, PartialEq)]
pub struct TitleDisplay {
    name: String,
    contents: String,
    color: Color,
    alignment: Alignment,
}

impl TitleDisplay {
    pub fn enumed(
        name: &str,
        color: Color,
        alignment: Alignment,
    ) -> Components {
        Components::TitleDisplay(TitleDisplay::new(name, color, alignment))
    }

    pub fn new(name: &str, color: Color, alignment: Alignment) -> TitleDisplay {
        TitleDisplay {
            name: name.to_string(),
            contents: String::new(),
            color,
            alignment,
        }
    }
}

impl Component for TitleDisplay {
    fn name(&self) -> &str {
        &self.name
    }

    fn handle(
        &mut self,
        state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>,
    ) {
        match e {
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name());
            },
            ComponentEvent::NowPlaying(id) => {
                self.contents = match id {
                    Some(id) => match &state.library.get(*id).unwrap().title {
                        Some(title) => title.to_string(),
                        None => "<Empty>".to_string(),
                    },
                    None => "<Unavailable>".to_string(),
                };
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::LostMpdConnection => {
                self.contents = "<Unavailable>".to_string();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, _h: u16, _focus: bool) {
        print!(
            "{}{}{}{}",
            color::Fg(self.color),
            cursor::Goto(x, y),
            self.contents.unicode_pad(w as usize, self.alignment, true),
            color::Fg(Color::Reset),
        );
    }
}
