/* TUI Component for a tag editor. It's big and not very customizable
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

use itertools::Itertools;
use mpd::song::Song;
use termion::{color, cursor, style};

use crate::color::Color;
use crate::components::{Component, Components};
use crate::event::*;
use crate::GlobalState;

#[derive(Debug, PartialEq)]
pub struct TagEditor {
    name: String,
    color: Color,
    songs: Vec<Song>,
    focus: usize,

    tags: Vec<(String, TagVal)>,
    sel: Option<usize>,
}

#[derive(PartialEq, Debug)]
enum TagVal {
    Some(String),
    None,
    Various,
}

impl TagVal {
    fn from(tag: &str, songs: &Vec<Song>) -> TagVal {
        if songs.is_empty() {
            return TagVal::None;
        }

        let first = songs[0].tags.get(tag);

        if songs.iter().all(|s| s.tags.get(tag) == first) {
            match first {
                None => TagVal::None,
                Some(val) => TagVal::Some(val.into()),
            }
        } else {
            TagVal::Various
        }
    }
}

impl std::fmt::Display for TagVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TagVal::Some(s) => s,
                TagVal::None => "<None>",
                TagVal::Various => "<Various>",
            }
        )
    }
}

impl TagEditor {
    pub fn enumed(name: &str, color: Color, songs: Vec<Song>) -> Components {
        Components::TagEditor(TagEditor::new(name, color, songs))
    }

    pub fn new(name: &str, color: Color, songs: Vec<Song>) -> TagEditor {
        let tags = vec![
            ("Artist".into(), TagVal::from("Artist", &songs)),
            ("Album".into(), TagVal::from("Album", &songs)),
        ];

        TagEditor {
            name: name.into(),
            color,
            songs,
            focus: 0,

            tags,
            sel: Some(0),
        }
    }

    pub fn select(&self, tx: mpsc::Sender<Event>) {
        if let Some(sel) = self.sel {
            let prompt = self.tags[sel].0.to_string();

            tx.send(Event::ToCommandLine(CommandLineEvent::RequestText(
                prompt,
            )))
            .unwrap();
        } else {
            self.save_tags(tx);
        }
    }

    fn save_tags(&self, tx: mpsc::Sender<Event>) {
        let mut tags = Vec::new();

        for (tag, val) in self.tags.iter() {
            match val {
                TagVal::Some(val) => {
                    tags.push((tag.to_string(), Some(val.to_string())))
                },
                TagVal::None => tags.push((tag.to_string(), None)),
                TagVal::Various => (),
            }
        }

        tx.send(Event::ToTagger(TaggerEvent::Tag(self.songs.clone(), tags)))
            .unwrap();
    }

    pub fn next(&mut self, tx: mpsc::Sender<Event>) {
        self.sel = if let Some(sel) = self.sel {
            if sel + 1 < self.tags.len() {
                Some(sel + 1)
            } else {
                None
            }
        } else {
            Some(0)
        };

        tx.send(self.spawn_needs_draw_event()).unwrap();
    }

    pub fn prev(&mut self, tx: mpsc::Sender<Event>) {
        self.sel = if let Some(sel) = self.sel {
            if sel == 0 {
                None
            } else {
                Some(sel - 1)
            }
        } else {
            Some(self.tags.len() - 1)
        };

        tx.send(self.spawn_needs_draw_event()).unwrap();
    }

    fn header(&self, x: u16, y: u16, w: u16) -> String {
        let mut header =
            self.songs.iter().map(|s| &s.file).format(", ").to_string();

        if header.len() > w.into() {
            header.truncate((w - 3).into());
            header.push_str("...");
        }

        format!("{}{}", cursor::Goto(x, y), header,)
    }

    fn tags(&self, x: u16, y: u16, w: u16, h: u16) -> String {
        let mut ret = String::new();

        let max_tag_len =
            self.tags.iter().map(|(tag, _)| tag.len()).max().unwrap();

        let max_val_len = self
            .tags
            .iter()
            .map(|(_, val)| val.to_string().len())
            .max()
            .unwrap();

        let bar_pos = if max_tag_len + max_val_len > w.into() {
            if w % 2 == 0 {
                (w / 2) - 1
            } else {
                w / 2
            }
        } else {
            max_tag_len as u16
        };

        for (i, (tag, val)) in self.tags.iter().enumerate() {
            if x + (i as u16) > h {
                break;
            }

            let focused = if let Some(sel) = self.sel {
                i == sel
            } else {
                false
            };

            ret.push_str(&self.tag(
                x,
                y + (i as u16),
                w,
                tag,
                bar_pos,
                val,
                focused,
            ));
        }

        ret
    }

    fn tag(
        &self,
        x: u16,
        y: u16,
        w: u16,
        tag: &str,
        bar_pos: u16,
        val: &TagVal,
        sel: bool,
    ) -> String {
        let left_len = bar_pos;
        let right_len = w - bar_pos;

        let mut tag = tag.to_string();
        tag.truncate(left_len.into());

        let mut val = val.to_string();
        val.truncate(right_len.into());

        let left_pad_len = std::cmp::max(0, left_len as usize - tag.len());
        let right_pad_len = std::cmp::max(0, right_len as usize - val.len());

        let s = format!(
            "{}{}{}│{}{}",
            cursor::Goto(x, y),
            tag,
            " ".repeat(left_pad_len as usize),
            val,
            " ".repeat(right_pad_len as usize),
        );

        if sel {
            format!("{}{}{}", style::Invert, s, style::NoInvert)
        } else {
            s
        }
    }

    fn save_button(&self, x: u16, y: u16) -> String {
        let s = format!("{}Save", cursor::Goto(x, y));

        if let None = self.sel {
            format!("{}{}{}", style::Invert, s, style::NoInvert,)
        } else {
            s
        }
    }
}

impl Component for TagEditor {
    fn name(&self) -> &str {
        &self.name
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        let tag_len: u16 = self.tags.len() as u16;

        self.clear(x, y, w, h);
        print!("{}", color::Fg(self.color));
        print!("{}", self.header(x, y, w));
        print!("{}{}", cursor::Goto(x, y + 1), "─".repeat((w).into()));
        print!("{}", self.tags(x, y + 2, w, h));
        print!("{}", self.save_button(x, y + 2 + tag_len));
    }

    fn handle(
        &mut self,
        _state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>,
    ) {
        match e {
            ComponentEvent::Next => self.next(tx),
            ComponentEvent::Prev => self.prev(tx),
            ComponentEvent::Select => self.select(tx),
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name())
            },
            ComponentEvent::ReturnText(s) => {
                let sel = self.sel.unwrap();

                let old_pair = &self.tags[sel];
                let new_pair =
                    (old_pair.0.clone(), TagVal::Some(s.to_string()));

                self.tags[sel] = new_pair;

                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            _ => (),
        }
    }
}
