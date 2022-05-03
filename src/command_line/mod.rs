/* Contains functionality related to in application command line
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

mod command;

use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem;
use std::sync::mpsc;
use termion::{clear, cursor, event::Key};

use crate::event::*;
use crate::mode::Mode;

#[derive(Debug)]
enum ContentType {
    Chars(String),
    Keys(Vec<Key>),
}

pub struct CommandLine {
    content: ContentType,
    statusline: String,
    text: String,
    mode: Mode,
    keybinds: HashMap<Vec<Key>, Event>,
    tx: mpsc::Sender<Event>,

    prompt: Option<String>,

    last_search: Option<String>,
    volume: i8,
}

impl CommandLine {
    pub fn new(tx: mpsc::Sender<Event>) -> CommandLine {
        CommandLine {
            content: ContentType::Keys(Vec::new()),
            statusline: "[----] Vol: %".into(),
            text: String::new(),
            mode: Mode::TUI,
            keybinds: HashMap::new(),
            tx,

            prompt: None,

            last_search: None,
            volume: -1,
        }
    }

    pub fn put_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn clear_text(&mut self) {
        self.text = "".to_string();
    }

    pub fn mode(&mut self, m: Mode) {
        self.clear();
        self.mode = m;

        self.content = match self.mode {
            Mode::TUI => ContentType::Keys(Vec::new()),
            Mode::Command | Mode::Search | Mode::GetText => {
                ContentType::Chars(String::new())
            },
        }
    }

    pub fn next_search(&mut self) {
        if let Some(last_search) = self.last_search.clone() {
            self.tx
                .send(Event::ToFocus(ComponentEvent::Search(last_search)))
                .unwrap();
        }
    }

    pub fn prev_search(&mut self) {
        if let Some(last_search) = self.last_search.clone() {
            self.tx
                .send(Event::ToFocus(ComponentEvent::SearchPrev(last_search)))
                .unwrap();
        }
    }

    pub fn vol_mv(&mut self, amount: i8) {
        // Force volume in range 0 <= vol <= 100
        let new_vol = min(100, max(0, self.volume + amount));

        self.tx
            .send(Event::ToMpd(MpdEvent::SetVolume(new_vol)))
            .unwrap();
    }

    pub fn input(&mut self, key: &Key) {
        self.clear_text();
        match (self.mode, &mut self.content) {
            (Mode::TUI, ContentType::Keys(keys)) => match key {
                Key::Char(':') => self.mode(Mode::Command),
                Key::Char('/') => self.mode(Mode::Search),
                Key::Esc => self.clear(),
                key => {
                    keys.push(key.clone());
                    if let Some(event) = self.keybinds.get(keys) {
                        self.tx.send(event.clone()).unwrap();
                        self.clear();
                    } else {
                        let has_match = self.keybinds.keys().any(|kb| {
                            keys.iter().zip(kb.iter()).all(|(a, b)| a == b)
                        });

                        if !has_match {
                            self.clear();
                        }
                    }
                },
            },
            (Mode::Command, ContentType::Chars(s))
            | (Mode::Search, ContentType::Chars(s))
            | (Mode::GetText, ContentType::Chars(s)) => match key {
                Key::Char('\n') => self.run(),
                Key::Esc => self.mode(Mode::TUI),
                Key::Backspace => {
                    if s.is_empty() {
                        self.mode(Mode::TUI);
                    } else {
                        s.pop();
                    }
                },
                Key::Char(c) => s.push(*c),
                _ => (),
            },
            bad => panic!("Command line reached invalid state {:?}", bad),
        }
    }

    pub fn clear(&mut self) {
        self.prompt = None;

        self.content = match self.content {
            ContentType::Chars(_) => ContentType::Chars(String::new()),
            ContentType::Keys(_) => ContentType::Keys(Vec::new()),
        }
    }

    pub fn bind(&mut self, keys: Vec<Key>, e: Event) {
        self.keybinds.insert(keys, e);
    }

    pub fn run(&mut self) {
        let content =
            mem::replace(&mut self.content, ContentType::Keys(Vec::new()));

        match (self.mode, content) {
            (Mode::Command, ContentType::Chars(cmd)) => {
                let args: Vec<&str> = cmd.split(" ").collect();

                match command::parse(&args) {
                    Some(e) => match e {
                        Event::ToCommandLine(CommandLineEvent::Echo(s)) => {
                            self.mode(Mode::TUI);
                            self.put_text(s.to_string());
                        },
                        e => {
                            self.tx.send(spawn_mode_event(Mode::TUI)).unwrap();
                            self.tx.send(e.clone()).unwrap();
                            self.tx.send(spawn_respond_event(&e)).unwrap();
                        },
                    },
                    None => {
                        self.mode(Mode::TUI);
                        self.tx.send(spawn_invalid_event(&cmd)).unwrap();
                    },
                };
            },
            (Mode::Search, ContentType::Chars(term)) => {
                self.last_search = Some(term.clone());
                self.mode(Mode::TUI);

                self.tx
                    .send(Event::ToFocus(ComponentEvent::Search(term.clone())))
                    .unwrap();
            },
            (Mode::GetText, ContentType::Chars(term)) => {
                self.tx
                    .send(Event::ToFocus(ComponentEvent::ReturnText(
                        term.clone(),
                    )))
                    .unwrap();
                self.mode(Mode::TUI);
            },
            (Mode::TUI, _) => panic!(
                "Invalid State: CommandLine called run while in Mode::TUI)"
            ),
            state => panic!("Invalid State: {:?}", state),
        }
    }

    pub fn draw(&self) {
        let (w, h) = termion::terminal_size().unwrap();

        let prompt = if let Some(s) = &self.prompt {
            format!("{}: ", s)
        } else {
            "Text: ".to_string()
        };

        print!(
            "{}{}{}{}{}{}",
            cursor::Goto(1, h),
            clear::CurrentLine,
            match self.mode {
                Mode::Command => ":",
                Mode::Search => "/",
                Mode::TUI => "",
                Mode::GetText => &prompt,
            },
            match &self.content {
                ContentType::Chars(cmd) => &cmd,
                ContentType::Keys(_) => &self.text,
            },
            cursor::Goto(w - (self.statusline.len() as u16), h),
            self.statusline,
        );
    }

    pub fn handle(&mut self, e: &CommandLineEvent, _tx: mpsc::Sender<Event>) {
        match e {
            CommandLineEvent::Echo(s) => self.put_text(s.to_string()),
            CommandLineEvent::RequestText(prompt, def) => {
                self.mode(Mode::GetText);
                self.prompt = Some(prompt.to_string());
                self.content =
                    ContentType::Chars(def.clone().unwrap_or("".to_owned()));
            },
            CommandLineEvent::NextSearch => self.next_search(),
            CommandLineEvent::PrevSearch => self.prev_search(),
            CommandLineEvent::Mode(m) => self.mode(*m),
            CommandLineEvent::SbrcError(line, msg) => self.put_text(format!(
                "sbrc: Invalid command at line {} '{}'",
                line,
                msg.to_string()
            )),
            CommandLineEvent::Input(key) => self.input(&key),
            CommandLineEvent::SbrcNotFound => {
                self.put_text("Sbrc not found. :q to quit.".to_string())
            },
            CommandLineEvent::MpdStatus(status) => {
                self.statusline = format!(
                    "[{}{}{}{}] Vol: {}%",
                    if status.repeat { "r" } else { "-" },
                    if status.random { "z" } else { "-" },
                    if status.single { "s" } else { "-" },
                    if status.consume { "c" } else { "-" },
                    status.volume,
                );
                self.volume = status.volume;
            },
            CommandLineEvent::VolumeMv(by) => self.vol_mv(*by),
            CommandLineEvent::VolumeUp(by) => self.vol_mv(max(0, *by)),
            CommandLineEvent::VolumeDown(by) => self.vol_mv(-(max(0, *by))),
        }
    }
}

pub fn run_headless(cmd: &str, tx: mpsc::Sender<Event>) -> Result<(), ()> {
    let cmd = cmd.split(" ").collect();
    match command::parse(&cmd) {
        Some(event) => {
            tx.send(event).unwrap();
            Ok(())
        },
        None => Err(()),
    }
}

fn spawn_invalid_event(cmd: &str) -> Event {
    Event::ToCommandLine(CommandLineEvent::Echo(format!(
        "Invalid Command '{}'",
        cmd
    )))
}

fn spawn_respond_event(e: &Event) -> Event {
    Event::ToCommandLine(CommandLineEvent::Echo(format!("Ran: {:?}", e)))
}

fn spawn_mode_event(mode: Mode) -> Event {
    Event::ToCommandLine(CommandLineEvent::Mode(mode))
}
