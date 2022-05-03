/* Contains functionality related to display color
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

use std::fmt::{Formatter, Result};
use termion::color as termionColor;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8, u8, u8),
    Reset,
}

impl termionColor::Color for Color {
    fn write_fg(&self, f: &mut Formatter) -> Result {
        match self {
            Color::Black => {
                write!(f, "{}", termionColor::Fg(termionColor::Black))
            },
            Color::Red => write!(f, "{}", termionColor::Fg(termionColor::Red)),
            Color::Green => {
                write!(f, "{}", termionColor::Fg(termionColor::Green))
            },
            Color::Yellow => {
                write!(f, "{}", termionColor::Fg(termionColor::Yellow))
            },
            Color::Blue => {
                write!(f, "{}", termionColor::Fg(termionColor::Blue))
            },
            Color::Magenta => {
                write!(f, "{}", termionColor::Fg(termionColor::Magenta))
            },
            Color::Cyan => {
                write!(f, "{}", termionColor::Fg(termionColor::Cyan))
            },
            Color::White => {
                write!(f, "{}", termionColor::Fg(termionColor::White))
            },
            Color::BrightBlack => {
                write!(f, "{}", termionColor::Fg(termionColor::LightBlack))
            },
            Color::BrightRed => {
                write!(f, "{}", termionColor::Fg(termionColor::LightRed))
            },
            Color::BrightGreen => {
                write!(f, "{}", termionColor::Fg(termionColor::LightGreen))
            },
            Color::BrightYellow => {
                write!(f, "{}", termionColor::Fg(termionColor::LightYellow))
            },
            Color::BrightBlue => {
                write!(f, "{}", termionColor::Fg(termionColor::LightBlue))
            },
            Color::BrightMagenta => {
                write!(f, "{}", termionColor::Fg(termionColor::LightMagenta))
            },
            Color::BrightCyan => {
                write!(f, "{}", termionColor::Fg(termionColor::LightCyan))
            },
            Color::BrightWhite => {
                write!(f, "{}", termionColor::Fg(termionColor::LightWhite))
            },
            Color::RGB(r, g, b) => write!(
                f,
                "{}",
                termionColor::Fg(termionColor::AnsiValue::rgb(*r, *g, *b))
            ),
            Color::Reset => {
                write!(f, "{}", termionColor::Fg(termionColor::Reset))
            },
        }
    }

    fn write_bg(&self, f: &mut Formatter) -> Result {
        match self {
            Color::Black => {
                write!(f, "{}", termionColor::Bg(termionColor::Black))
            },
            Color::Red => write!(f, "{}", termionColor::Bg(termionColor::Red)),
            Color::Green => {
                write!(f, "{}", termionColor::Bg(termionColor::Green))
            },
            Color::Yellow => {
                write!(f, "{}", termionColor::Bg(termionColor::Yellow))
            },
            Color::Blue => {
                write!(f, "{}", termionColor::Bg(termionColor::Blue))
            },
            Color::Magenta => {
                write!(f, "{}", termionColor::Bg(termionColor::Magenta))
            },
            Color::Cyan => {
                write!(f, "{}", termionColor::Bg(termionColor::Cyan))
            },
            Color::White => {
                write!(f, "{}", termionColor::Bg(termionColor::White))
            },
            Color::BrightBlack => {
                write!(f, "{}", termionColor::Bg(termionColor::LightBlack))
            },
            Color::BrightRed => {
                write!(f, "{}", termionColor::Bg(termionColor::LightRed))
            },
            Color::BrightGreen => {
                write!(f, "{}", termionColor::Bg(termionColor::LightGreen))
            },
            Color::BrightYellow => {
                write!(f, "{}", termionColor::Bg(termionColor::LightYellow))
            },
            Color::BrightBlue => {
                write!(f, "{}", termionColor::Bg(termionColor::LightBlue))
            },
            Color::BrightMagenta => {
                write!(f, "{}", termionColor::Bg(termionColor::LightMagenta))
            },
            Color::BrightCyan => {
                write!(f, "{}", termionColor::Bg(termionColor::LightCyan))
            },
            Color::BrightWhite => {
                write!(f, "{}", termionColor::Bg(termionColor::LightWhite))
            },
            Color::RGB(r, g, b) => write!(
                f,
                "{}",
                termionColor::Bg(termionColor::AnsiValue::rgb(*r, *g, *b))
            ),
            Color::Reset => {
                write!(f, "{}", termionColor::Bg(termionColor::Reset))
            },
        }
    }
}
