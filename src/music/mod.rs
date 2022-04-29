/* Functionality related to mpd
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

use mpd::Client;

pub mod mpd_listener;
pub mod mpd_sender;

fn get_mpd_conn(ip: &str, port: &str) -> Option<Client> {
    match Client::connect(format!("{}:{}", ip, port)) {
        Ok(conn) => Some(conn),
        _ => None,
    }
}
