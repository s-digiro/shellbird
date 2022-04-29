/* Interface and shared functionality for a splitter TUI component
   which holds other TUI Components
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

use crate::components::*;
use crate::GlobalState;

mod vector_splitter;

pub use vector_splitter::HorizontalSplitter;
pub use vector_splitter::VerticalSplitter;

#[derive(PartialEq)]
pub enum MoveFocusResult {
    Success,
    Fail,
}

#[derive(PartialEq, Debug)]
pub enum Size {
    Percent(u8),
    Absolute(u16),
    Remainder,
}

#[derive(Debug, PartialEq)]
pub struct Panel {
    size: Size,
    key: String,
}

impl Panel {
    pub fn new(size: Size, key: String) -> Panel {
        Panel { size, key }
    }
}

pub trait Splitter: Component {
    fn focus(&self) -> Option<&str>;

    fn next(&mut self) -> MoveFocusResult;
    fn prev(&mut self) -> MoveFocusResult;

    fn contains(&self, key: &str) -> bool;
    fn children(&self) -> Vec<&str>;
}

#[derive(Debug, PartialEq)]
pub enum Splitters {
    VerticalSplitter(VerticalSplitter),
    HorizontalSplitter(HorizontalSplitter),
}

impl Component for Splitters {
    fn handle(&mut self, state: &GlobalState, e: &ComponentEvent, tx: mpsc::Sender<Event>) {
        match self {
            Splitters::VerticalSplitter(c) => c.handle(state, e, tx),
            Splitters::HorizontalSplitter(c) => c.handle(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        match self {
            Splitters::VerticalSplitter(c) => c.draw(x, y, w, h, focus),
            Splitters::HorizontalSplitter(c) => c.draw(x, y, w, h, focus),
        }
    }

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        match self {
            Splitters::VerticalSplitter(c) => c.border(x, y, w, h),
            Splitters::HorizontalSplitter(c) => c.border(x, y, w, h),
        }
    }

    fn name(&self) -> &str {
        match self {
            Splitters::VerticalSplitter(c) => c.name(),
            Splitters::HorizontalSplitter(c) => c.name(),
        }
    }
}

impl Splitter for Splitters {
    fn focus(&self) -> Option<&str> {
        match self {
            Splitters::VerticalSplitter(c) => c.focus(),
            Splitters::HorizontalSplitter(c) => c.focus(),
        }
    }

    fn next(&mut self) -> MoveFocusResult {
        match self {
            Splitters::VerticalSplitter(c) => c.next(),
            Splitters::HorizontalSplitter(c) => c.next(),
        }
    }

    fn prev(&mut self) -> MoveFocusResult {
        match self {
            Splitters::VerticalSplitter(c) => c.prev(),
            Splitters::HorizontalSplitter(c) => c.prev(),
        }
    }
    fn contains(&self, key: &str) -> bool {
        match self {
            Splitters::VerticalSplitter(c) => c.contains(key),
            Splitters::HorizontalSplitter(c) => c.contains(key),
        }
    }

    fn children(&self) -> Vec<&str> {
        match self {
            Splitters::VerticalSplitter(c) => c.children(),
            Splitters::HorizontalSplitter(c) => c.children(),
        }
    }
}
