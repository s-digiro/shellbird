use std::sync::mpsc;
use termion::cursor;

use crate::GlobalState;
use crate::components::{Component, ErrorBox};
use crate::event::*;
use super::{Panel, Splitter, Size, MoveFocusResult};

mod horizontal_splitter;
mod vertical_splitter;

pub use horizontal_splitter::HorizontalSplitter;
pub use vertical_splitter::VerticalSplitter;

#[derive(Debug)]
#[derive(PartialEq)]
struct VectorSplitter {
    name: String,
    panels: Vec<Panel>,
    sel: usize,
    draw_borders: bool,
}

impl Splitter for VectorSplitter {
    fn contains(&self, key: &str) -> bool {
        self.panels.iter()
            .map(|p| p.key.as_str())
            .collect::<Vec<&str>>()
            .contains(&key)
    }

    fn children(&self) -> Vec<&str> {
        self.panels.iter()
            .map(|p| p.key.as_str())
            .collect()
    }

    fn focus(&self) -> Option<&str> {
        if let Some(panel) = self.panels.get(self.sel) {
            Some(&panel.key)
        } else {
            None
        }
    }

    fn next(&mut self) -> MoveFocusResult {
        if self.sel + 1 < self.panels.len() {
            self.sel = self.sel + 1;
            MoveFocusResult::Success
        } else {
            MoveFocusResult::Fail
        }
    }

    fn prev(&mut self) -> MoveFocusResult {
        if self.sel as i32 - 1 >= 0 {
            self.sel = self.sel - 1;
            MoveFocusResult::Success
        } else {
            MoveFocusResult::Fail
        }
    }
}

impl Component for VectorSplitter {
    fn name(&self) -> &str { &self.name }

    fn draw(&self,x: u16, y: u16, w: u16, h: u16, focus: bool) {
        ErrorBox::new().draw(x, y, w, h, focus);
    }
}

fn draw_vertical_line(x: u16, y: u16, h: u16) {
    for i in 0..h {
        print!("{}│", cursor::Goto(x, y + i));
    }
    print!("{}┬", cursor::Goto(x, y));
    print!("{}┴", cursor::Goto(x, y + h - 1));
}

fn draw_horizontal_line(x: u16, y: u16, w: u16) {
    for i in 0..w {
        print!("{}─", cursor::Goto(x + i, y));
    }
    print!("{}├", cursor::Goto(x, y));
    print!("{}┤", cursor::Goto(x + w - 1, y));
}

fn draw_right_border(x: u16, y: u16, h: u16) {
    for i in 0..h {
        print!("{}│", cursor::Goto(x, y + i));
    }
    print!("{}┐", cursor::Goto(x, y));
    print!("{}┘", cursor::Goto(x, h + y - 1));
}

fn draw_bottom_border(x: u16, y: u16, w: u16) {
    for i in 0..w {
        print!("{}─", cursor::Goto(x + i, y));
    }
    print!("{}└", cursor::Goto(x, y));
    print!("{}┘", cursor::Goto(w + x - 1, y));
}
