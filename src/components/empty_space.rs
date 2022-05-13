/* TUI Component for empty space
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

use crate::components::{Component, Components};

#[derive(Debug, PartialEq)]
pub struct EmptySpace {
    name: String,
}

impl EmptySpace {
    pub fn enumed(name: &str) -> Components {
        Components::EmptySpace(EmptySpace::new(name))
    }

    pub fn new(name: &str) -> EmptySpace {
        EmptySpace {
            name: name.to_string(),
        }
    }
}

impl Component for EmptySpace {
    fn name(&self) -> &str {
        &self.name
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        self.clear(x, y, w, h);
    }
}
