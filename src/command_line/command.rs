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

use crate::event::*;

pub fn replace_macros(cmd: Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = cmd.clone().iter()
        .map(|s| s.to_string()).collect();

    for (i, token) in cmd.iter().enumerate() {
        match *token {
            "<space>" => ret[i] = String::from(" "),
            _ => (),
        }
    }

    ret
}

pub fn parse(cmd: &Vec<&str>) -> Option<Event> {
    let cmd = replace_macros(cmd.clone());

    match get_lowercase(&cmd, 0) {
        Some(s) => match s.as_str() {
            "echo" => match cmd.get(1) {
                Some(s) => Some(Event::ToCommandLine(CommandLineEvent::Echo(s.to_string()))),
                None => None,
            },

            "draw" => draw(&cmd),

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
            "next" => Some(Event::ToFocus(ComponentEvent::Next)),
            "prev" => Some(Event::ToFocus(ComponentEvent::Prev)),
            "select" => Some(Event::ToFocus(ComponentEvent::Select)),
            "start" => Some(Event::ToFocus(ComponentEvent::Start)),

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
            | "s" => match get_lowercase(&cmd, 1) {
                Some(s) => Some(Event::ToFocus(ComponentEvent::Search(s.to_string()))),
                None => None,
            },

            "goto"
            | "go"
            | "g"
            | "to" => match get_usize(&cmd, 1) {
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
            | "bindkey" => match cmd.get(1) {
                Some(s) => {
                    let new_cmd = cmd.iter()
                        .skip(2)
                        .map(|s| s.as_str())
                        .collect();

                    match parse(&new_cmd) {
                        Some(e) => match NestableEvent::from_event(e) {
                            Some(e) => Some(Event::BindKey(s.to_string(), e)),
                            None => None,
                        },
                        _ => None,
                    }
                },
                None => None,
            },

            _ => None,
        }
        None => None,
    }
}

fn get_lowercase(cmd: &Vec<String>, i: usize) -> Option<String> {
    match cmd.get(i) {
        Some(s) => Some(s.to_string().to_lowercase()),
        None => None,
    }
}

fn get_usize(cmd: &Vec<String>, i: usize) -> Option<usize> {
    match cmd.get(i) {
        Some(s) => match s.parse::<usize>() {
            Ok(num) => Some(num),
            _ => None,
        },
        None => None,
    }
}

fn _get_boolean(cmd: &Vec<String>, i: usize) -> Option<bool> {
    match cmd.get(i) {
        Some(s) => match s.to_lowercase().as_str() {
            "0" | "false" => Some(false),
            _ => Some(true),
        },
        None => None,
    }
}

fn draw(cmd: &Vec<String>) -> Option<Event> {
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
