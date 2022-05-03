/* TUI Component for Menus which list track queue
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

use mpd::Song;
use std::sync::mpsc;

use crate::color::Color;
use crate::components::{menu::Menu, Component, Components};
use crate::event::*;
use crate::GlobalState;
use termion::{color, cursor, style};
use unicode_truncate::{Alignment, UnicodeTruncateStr};

#[derive(Debug, PartialEq)]
pub struct Queue {
    tracks: Vec<Song>,
    menu: Menu,
    now_playing: Option<Song>,
}

impl Queue {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> Components {
        Components::Queue(Queue::new(
            name,
            color,
            focus_color,
            title,
            title_alignment,
            menu_alignment,
        ))
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> Queue {
        Queue {
            tracks: Vec::new(),
            now_playing: None,
            menu: Menu {
                title,
                name: name.to_string(),
                focus_color,
                color,
                selection: 0,
                items: Vec::new(),
                title_alignment,
                menu_alignment,
            },
        }
    }

    fn set_now_playing(&mut self, target: &Option<Song>) {
        match target {
            Some(target) => self.now_playing = Some(target.clone()),
            None => self.now_playing = None,
        }
    }

    fn update_items(&mut self, tracks: &Vec<Song>) {
        self.tracks = tracks.clone();
        self.update_menu_items();
    }

    fn update_menu_items(&mut self) {
        self.menu.items = self
            .tracks
            .iter()
            .map(|s| match &s.title {
                Some(title) => title.to_string(),
                None => "<Empty>".to_string(),
            })
            .collect();
    }

    fn selected_tracks(&self) -> Vec<Song> {
        if let Some(track) = self.tracks.get(self.menu.selection) {
            vec![track.clone()]
        } else {
            Vec::new()
        }
    }
}

impl Component for Queue {
    fn name(&self) -> &str {
        &self.menu.name
    }

    fn handle(
        &mut self,
        _state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>,
    ) {
        match e {
            ComponentEvent::OpenTags => {
                tx.send(Event::ToApp(AppEvent::TagUI(self.selected_tracks())))
                    .unwrap();
            },
            ComponentEvent::Start => (),
            ComponentEvent::Next => {
                self.menu.next();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Prev => {
                self.menu.prev();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToTop => {
                self.menu.to_top();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToBottom => {
                self.menu.to_bottom();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoTo(i) => {
                self.menu.to(*i);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Search(s) => {
                self.menu.search(s);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::SearchPrev(s) => {
                self.menu.search_prev(s);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Select => {
                if let Some(song) = self.tracks.get(self.menu.selection) {
                    tx.send(Event::ToMpd(MpdEvent::PlayAt(song.clone())))
                        .unwrap()
                }
            },
            ComponentEvent::NowPlaying(song) => {
                self.set_now_playing(&song);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Queue(q) => {
                self.update_items(q);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::LostMpdConnection => {
                self.now_playing = None;
                self.update_items(&Vec::new());
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name());
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        let mut cur_y = y;

        let mut buffer = String::new();

        if let Some(title) = &self.menu.title {
            buffer.push_str(&format!(
                "{}{}{}{}{}{}",
                color::Fg(self.menu.color(focus)),
                cursor::Goto(x, y),
                title.unicode_pad(w as usize, self.menu.title_alignment, true),
                cursor::Goto(x, y + 1),
                "─".repeat(w as usize),
                style::Reset,
            ));

            cur_y = cur_y + 2;
        }

        let mut i = self.menu.first_visible(h);
        for line in cur_y..(y + h) {
            if let Some(s) = self.menu.items.get(i) {
                let s =
                    s.unicode_pad(w as usize, self.menu.menu_alignment, true);

                if self.menu.selection == i {
                    buffer.push_str(&format!("{}", style::Invert));
                }

                if let Some(np) = &self.now_playing {
                    if self.tracks.get(i) == Some(np) {
                        buffer.push_str(&format!("{}", style::Bold));
                    }
                }

                buffer.push_str(&format!(
                    "{}{}{}{}",
                    color::Fg(self.menu.color(focus)),
                    cursor::Goto(x, line),
                    s,
                    style::Reset,
                ));
            } else {
                buffer.push_str(&format!(
                    "{}{}",
                    cursor::Goto(x, line),
                    " ".repeat(w as usize),
                ));
            }

            i = i + 1;
        }

        buffer.push_str(&format!("{}", style::Reset));

        print!("{}", buffer);
    }
}
