/* Events sent to and handled by Tagger Thread
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

use mpd::Song;

#[derive(Debug, Clone)]
pub enum TaggerEvent {
    // Instruct Tagger to use String as path to directory to look for files in
    MusicDir(String),

    // Instruct Tagger tag songs in Vec<Song> with tag pairs in Vec<(String,
    // Option<String>)>
    Tag(Vec<Song>, Vec<(String, Option<String>)>),

    // Instruct Tagger to use String as path to directory to store temp files
    // in
    TempDir(String),
}
