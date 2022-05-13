/* Events sent to and handled by CommandLine
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

use crate::color::Color;
use termion::event::Key;

#[derive(Debug, Clone)]
pub enum CommandLineEvent {
    // Print text to statusline
    Echo(String),

    // Send Input to be processed by commandline
    Input(Key),

    // Update commandline status to reflect MPD's status
    MpdStatus(mpd::status::Status),

    // Redo last search in forwards direction
    NextSearch,

    // Redo last search in backwards direction
    PrevSearch,

    // Request text from commandline to be sent back to focused component.
    // Also sends prompt and optional default value
    RequestText(String, Option<String>),

    // Sets color of commandline text
    SetColor(Color),

    // Report to commandline that there was an error when parsing Sbrc file.
    // This probably could just be an App::Error enum, actually
    SbrcError(usize, String),

    // Report to commandline that Sbrc is not found
    SbrcNotFound,

    // Move volume down by i8. Must be positive
    VolumeDown(i8),

    // Move volume by i8, can be negative or positive
    VolumeMv(i8),

    // Move volume up by i8, must be positive
    VolumeUp(i8),
}
