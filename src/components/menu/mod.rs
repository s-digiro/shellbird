/* TUI Component for Menus
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

pub mod playlist_menu;
pub mod queue;
pub mod style_menu;
pub mod tag_menu;
pub mod track_menu;

use crate::color::Color;
use termion::{color, cursor, style};
use unicode_truncate::{Alignment, UnicodeTruncateStr};

#[derive(Debug, PartialEq)]
pub struct Menu {
    pub name: String,
    pub selection: usize,
    pub items: Vec<String>,
    pub color: Color,
    pub focus_color: Color,
    pub title: Option<String>,
    pub title_alignment: Alignment,
    pub menu_alignment: Alignment,
}

impl Menu {
    pub fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        let mut cur_y = y;

        let mut buffer = String::new();

        let mut i = self.first_visible(h);

        if let Some(title) = &self.title {
            buffer.push_str(&format!(
                "{}{}{}{}{}{}",
                color::Fg(self.color(focus)),
                cursor::Goto(x, y),
                title.unicode_pad(w as usize, self.title_alignment, true),
                cursor::Goto(x, y + 1),
                "─".repeat(w as usize),
                style::Reset,
            ));

            cur_y = cur_y + 2;
        }

        buffer.push_str(&format!("{}", color::Fg(self.color(focus))));

        for line in cur_y..(y + h) {
            if let Some(s) = self.items.get(i) {
                if self.selection == i {
                    buffer.push_str(&format!(
                        "{}{}{}{}{}",
                        style::Invert,
                        cursor::Goto(x, line),
                        s.unicode_pad(w as usize, self.menu_alignment, true),
                        style::Reset,
                        color::Fg(self.color(focus)),
                    ));
                } else {
                    buffer.push_str(&format!(
                        "{}{}",
                        cursor::Goto(x, line),
                        s.unicode_pad(w as usize, self.menu_alignment, true),
                    ));
                }
            } else {
                buffer.push_str(&format!(
                    "{}{}",
                    cursor::Goto(x, line),
                    " ".unicode_pad(w as usize, self.menu_alignment, true),
                ));
            }

            i = i + 1;
        }

        buffer.push_str(&format!("{}", style::Reset));

        print!("{}", buffer);
    }
    pub fn color(&self, focus: bool) -> Color {
        if focus {
            self.focus_color
        } else {
            self.color
        }
    }

    pub fn selection(&self) -> Option<&String> {
        match self.items.get(self.selection) {
            Some(item) => Some(item),
            None => None,
        }
    }

    pub fn to_top(&mut self) {
        self.selection = 0;
    }

    pub fn to_bottom(&mut self) {
        self.selection = std::cmp::max(0, self.items.len() - 1);
    }

    pub fn to(&mut self, i: usize) {
        if i < self.items.len() {
            self.selection = i;
        }
    }

    pub fn search(&mut self, s: &str) {
        self.selection = *self
            .items
            .iter()
            .enumerate()
            .skip(self.selection + 1)
            .filter(|(_, item)| item.to_lowercase().contains(&s.to_lowercase()))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .first()
            .unwrap_or(&self.selection);
    }

    pub fn search_prev(&mut self, s: &str) {
        let len = self.items.len();

        self.selection = *self
            .items
            .iter()
            .enumerate()
            .rev()
            .skip(len - self.selection)
            .filter(|(_, item)| item.to_lowercase().contains(&s.to_lowercase()))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .first()
            .unwrap_or(&self.selection);
    }

    pub fn next(&mut self) {
        self.selection = if self.items.len() == 0 {
            0
        } else if self.selection + 1 >= self.items.len() {
            0
        } else {
            self.selection + 1
        }
    }

    pub fn prev(&mut self) {
        self.selection = if self.items.len() == 0 {
            0
        } else if self.selection <= 0 {
            self.items.len() - 1
        } else {
            self.selection - 1
        }
    }

    pub fn first_visible(&self, h: u16) -> usize {
        let h = match self.title {
            Some(_) => h - 2,
            None => h,
        };

        let mut center = h / 2;
        if h % 2 == 0 {
            center = center - 1;
        }

        // If item is close to top
        if self.selection <= center as usize {
            0
        // If item is close to bottom
        } else if self.selection as i32
            >= self.items.len() as i32 - center as i32
        {
            // Set first drawn item to either 0 or half screen above middle
            std::cmp::max(0, self.items.len() as i32 - h as i32) as usize
        // If item is in middle
        } else {
            (self.selection - center as usize) as usize
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Parent {
    parent: Option<String>,
}

impl Parent {
    pub fn new(parent: Option<String>) -> Parent {
        Parent { parent }
    }

    pub fn is(&self, target: &str) -> bool {
        match &self.parent {
            Some(parent) if parent == target => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self.parent {
            Some(_) => false,
            None => true,
        }
    }
}
