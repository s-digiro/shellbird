/* TUI Component for displaying a tag on the currently playing track
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

use std::sync::mpsc;
use termion::{cursor, color};
use unicode_truncate::{UnicodeTruncateStr, Alignment};
use crate::GlobalState;
use crate::color::Color;
use crate::event::*;
use crate::components::{Component, Components};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TagDisplay {
    name: String,
    tag: String,
    contents: String,
    color: Color,
    alignment: Alignment,
}

impl TagDisplay {
    pub fn enumed(
        name: &str,
        color: Color,
        alignment: Alignment,
        tag: &str
    ) -> Components {
        Components::TagDisplay(
            TagDisplay::new(name, color, alignment, tag)
        )
    }

    pub fn new(
        name: &str,
        color: Color,
        alignment: Alignment,
        tag: &str
    ) -> TagDisplay {
        TagDisplay {
            name: name.to_string(),
            tag: tag.to_string(),
            contents: String::new(),
            color,
            alignment,
        }
    }
}

impl Component for TagDisplay {
    fn name(&self) -> &str { &self.name }

    fn handle(
        &mut self,
        _state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name());
            },
            ComponentEvent::NowPlaying(song) => {
                self.contents = match song {
                    Some(song) => match song.tags.get(&self.tag) {
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
            color::Fg(color::Reset),
        );
    }
}
