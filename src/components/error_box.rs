/* TUI Component which indicates an error
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

use crate::components::{Component, Components};
use termion::color;
use termion::cursor;

#[derive(Debug, PartialEq)]
pub struct ErrorBox;

impl ErrorBox {
    pub fn enumed() -> Components {
        Components::ErrorBox(ErrorBox::new())
    }

    pub fn new() -> ErrorBox {
        ErrorBox {}
    }
}

impl Component for ErrorBox {
    fn name(&self) -> &str {
        "Error"
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        let mut text = "Error".to_string();
        text.truncate((w - 2).into());

        print!("{}", color::Fg(color::Red));
        self.border(x, y, w, h);
        print!(
            "{}{}{}",
            cursor::Goto(x + 1, y + 1),
            text,
            color::Fg(color::Reset),
        )
    }
}
