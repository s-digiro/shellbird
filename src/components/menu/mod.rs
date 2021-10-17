pub mod queue;
pub mod playlist_menu;
pub mod tag_menu;
pub mod track_menu;
pub mod style_menu;

use std::sync::mpsc;
use crate::components::Component;
use crate::event::*;
use crate::color::Color;
use crate::GlobalState;
use termion::{cursor, style, color};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Menu {
    pub selection: usize,
    pub items: Vec<String>,
    pub color: Color,
}

impl Component for Menu {
    fn name(&self) -> &str { "Menu" }

    fn handle_focus(
        &mut self,
        _state: &GlobalState,
        request: &FocusEvent,
        _tx: mpsc::Sender<Event>
    ) {
        match request {
            FocusEvent::Next => self.next(),
            FocusEvent::Prev => self.prev(),
            FocusEvent::GoToTop => self.selection = 0,
            FocusEvent::GoToBottom => self.selection = std::cmp::max(0, self.items.len() - 1),
            FocusEvent::GoTo(i) if *i < self.items.len() => self.selection = *i,
            FocusEvent::Search(s) =>
                self.selection = *self.items.iter().enumerate()
                    .skip(self.selection + 1)
                    .filter(|(_, item)| item.to_lowercase().contains(&s.to_lowercase()))
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>()
                    .first()
                    .unwrap_or(&self.selection),
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        let first_visible = self.first_visible(h);

        let mut line = y;
        for (i, val) in self.items.iter().enumerate().skip(first_visible) {
            let mut val = val.clone();

            utf8_truncate(&mut val, w as usize);

            println!("{}", color::Fg(self.color));
            if self.selection == i {
                print!("{}", style::Invert);
            }

            print!("{}{}{}{}",
                cursor::Goto(x, line),
                val,
                " ".repeat(w as usize - val.len()),
                style::Reset,
            );

            line += 1;

            if line >= y + h {
                break;
            }
        }
    }
}

impl Menu {
    pub fn selection(&self) -> Option<&String> {
        match self.items.get(self.selection) {
            Some(item) => Some(item),
            None => None,
        }
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

    fn prev(&mut self) {
        self.selection = if self.items.len() == 0 {
            0
        } else if self.selection <= 0 {
            self.items.len() - 1
        } else {
            self.selection - 1
        }
    }

    pub fn first_visible(
        &self,
        h: u16,
    ) -> usize {
        let mut center = h / 2;
        if h % 2 == 0 {
            center = center - 1;
        }

        // If item is close to top
        if self.selection <= center as usize {
            0
        // If item is close to bottom
        } else if self.selection as i32 >= self.items.len() as i32 - center as i32 {
            // Set first drawn item to either 0 or half screen above middle
            std::cmp::max(
                0,
                self.items.len() as i32 - h as i32
            ) as usize
        // If item is in middle
        } else {
            (self.selection - center as usize) as usize
        }
    }

}

#[derive(Debug)]
#[derive(PartialEq)]
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

fn utf8_truncate(input : &mut String, maxsize: usize) {
    let mut utf8_maxsize = input.len();
    if utf8_maxsize >= maxsize {
        {
            let mut char_iter = input.char_indices();
            while utf8_maxsize >= maxsize {
                utf8_maxsize = match char_iter.next_back() {
                    Some((index, _)) => index,
                    _ => 0
                };
            }
        } // Extra {} wrap to limit the immutable borrow of char_indices()
        input.truncate(utf8_maxsize);
    }
}

