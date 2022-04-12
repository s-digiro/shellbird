/* Contains functionality related to in-application commands
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

use std::collections::HashMap;

use crate::event::*;

use termion::event::Key;

lazy_static! {
    static ref SYM_MAP: HashMap<&'static str, Key> = {
        let mut m = HashMap::new();
        m.insert("<space>", Key::Char(' '));
        m.insert("<return>", Key::Char('\n'));
        m.insert("<enter>", Key::Char('\n'));
        m
    };
}

pub fn str_to_keys(s: &str) -> Vec<Key> {
    eprintln!("Called str_to_keys");
    let mut ret = Vec::new();

    let mut sym = String::new();

    for c in s.chars() {
        eprintln!("c: {}", c);
        eprintln!("ret: {:?}", ret);
        eprintln!("sym: {}", sym);
        eprintln!();
        if !sym.is_empty() {
            sym.push(c);
            if c == '>' {
                eprintln!("Found >, checking map");
                for key in SYM_MAP.keys() {
                    eprintln!("{:?}: {:?}", key, SYM_MAP.get(key));
                }
                eprintln!("sym.as_str(): {:?}", sym.as_str());
                let val = SYM_MAP.get(sym.as_str());
                eprintln!("val: {:?}", val);
                match SYM_MAP.get(sym.as_str()) {
                    Some(key) => ret.push(*key),
                    None => sym.chars().for_each(|c| ret.push(Key::Char(c))),
                }
                sym.clear();
            }
        } else if c == '<' {
            sym.push(c);
        } else {
            ret.push(Key::Char(c));
        }
    }

    eprintln!("ret: {:?}", ret);
    eprintln!("sym: {}", sym);


    ret
}

pub fn parse(cmd: &Vec<&str>) -> Option<Event> {
    match get_lowercase(cmd, 0) {
        Some(s) => match s.as_str() {
            "echo" => match cmd.get(1) {
                Some(s) => Some(Event::ToCommandLine(CommandLineEvent::Echo(s.to_string()))),
                None => None,
            },

            "draw" => draw(cmd),

            "quit"
            | "q"
            | "exit" => Some(Event::ToApp(AppEvent::Quit)),

            "switchscreen"
            | "screen" => match cmd.get(1) {
                Some(s) => Some(Event::ToApp(AppEvent::SwitchScreen(s.to_string()))),
                None => None,
            },

            "focusnext" => Some(Event::ToScreen(ScreenEvent::FocusNext)),
            "focusprev" => Some(Event::ToScreen(ScreenEvent::FocusPrev)),
            "down" => Some(Event::ToFocus(ComponentEvent::Next)),
            "up" => Some(Event::ToFocus(ComponentEvent::Prev)),
            "select" => Some(Event::ToFocus(ComponentEvent::Select)),
            "start" => Some(Event::ToFocus(ComponentEvent::Start)),

            "next" => Some(Event::ToMpd(MpdEvent::Next)),
            "prev" => Some(Event::ToMpd(MpdEvent::Prev)),

            "top"
            | "gotop"
            | "gototop"
            | "totop" => Some(Event::ToFocus(ComponentEvent::GoToTop)),

            "bottom"
            | "gobottom"
            | "gotobottom"
            | "tobottom"
            | "bot"
            | "gobot"
            | "gotobot"
            | "tobot" => Some(Event::ToFocus(ComponentEvent::GoToBottom)),

            "search"
            | "s" => match get_lowercase(cmd, 1) {
                Some(s) => Some(Event::ToFocus(ComponentEvent::Search(s.to_string()))),
                None => None,
            },
            "prevsearch" => Some(Event::ToCommandLine(CommandLineEvent::PrevSearch)),
            "nextsearch" => Some(Event::ToCommandLine(CommandLineEvent::NextSearch)),

            "goto"
            | "go"
            | "g"
            | "to" => match get_usize(cmd, 1) {
                Some(num) => Some(Event::ToFocus(ComponentEvent::GoTo(num))),
                None => None,
            }

            "togglepause"
            | "pause"
            | "toggle" => Some(Event::ToMpd(MpdEvent::TogglePause)),

            "clear"
            | "clearqueue" => Some(Event::ToMpd(MpdEvent::ClearQueue)),

            "random" => Some(Event::ToMpd(MpdEvent::Random)),

            "bind"
            | "bindkey" => cmd.get(1)
                .and_then(|key| {
                    eprintln!("Parsing a bind");
                    let keybind = str_to_keys(key);

                    let cmd = cmd.iter()
                        .skip(2)
                        .map(|s| *s)
                        .collect();
                    
                    parse(&cmd)
                        .and_then(|e| NestableEvent::from_event(e))
                        .and_then(|ne| Some(Event::BindKey(keybind, ne)))
                }),

            _ => None,
        }
        None => None,
    }
}

fn get_lowercase(cmd: &Vec<&str>, i: usize) -> Option<String> {
    match cmd.get(i) {
        Some(s) => Some(s.to_string().to_lowercase()),
        None => None,
    }
}

fn get_usize(cmd: &Vec<&str>, i: usize) -> Option<usize> {
    match cmd.get(i) {
        Some(s) => match s.parse::<usize>() {
            Ok(num) => Some(num),
            _ => None,
        },
        None => None,
    }
}

fn _get_boolean(cmd: &Vec<&str>, i: usize) -> Option<bool> {
    match cmd.get(i) {
        Some(s) => match s.to_lowercase().as_str() {
            "0" | "false" => Some(false),
            _ => Some(true),
        },
        None => None,
    }
}

fn draw(cmd: &Vec<&str>) -> Option<Event> {
    if let Some(component) = cmd.get(1) {
        let (max_w, max_h) = termion::terminal_size().unwrap();

        let max_h = max_h - 1;

        let x = get_usize(cmd, 2).unwrap_or(1) as u16;
        let y = get_usize(cmd, 3).unwrap_or(1) as u16;
        let w = get_usize(cmd, 4).unwrap_or(max_w as usize) as u16;
        let h = get_usize(cmd, 5).unwrap_or(max_h as usize) as u16;
        let focus = match cmd.get(6) {
            Some(s) => s.to_string(),
            None => "<None>".to_string(),
        };

        Some(
            Event::ToComponent(
                component.to_string(),
                ComponentEvent::Draw(x, y, w, h, focus)
            )
        )
    } else {
        None
    }
}
