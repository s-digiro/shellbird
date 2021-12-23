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

use std::collections::HashMap;
use std::sync::mpsc;
use termion::{cursor, clear, event::Key};

use crate::event::*;
use crate::mode::Mode;

pub struct CommandLine {
    contents: String,
    statusline: String,
    text: String,
    mode: Mode,
    keybinds: HashMap<String, Event>,
    tx: mpsc::Sender<Event>,
}

impl CommandLine {
    pub fn new(tx: mpsc::Sender<Event>) -> CommandLine {
        CommandLine {
            contents: String::new(),
            statusline: String::new(),
            text: String::new(),
            mode: Mode::TUI,
            keybinds: HashMap::new(),
            tx,
        }
    }

    pub fn put_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn clear_text(&mut self) {
        self.text = "".to_string();
    }

    pub fn back(&mut self) -> Option<Event> {
        if self.contents.is_empty() {
            Some(spawn_mode_event(Mode::TUI))
        } else {
            self.contents.pop();
            None
        }
    }

    pub fn mode(&mut self, m: Mode) {
        self.clear();
        self.mode = m;
    }


    pub fn add(&mut self, c: char) {
        self.clear_text();
        match self.mode {
            Mode::TUI => {
                if c != '\n' {
                    self.contents.push(c);
                }

                if let Some(event) = self.keybinds.get(&self.contents) {
                    self.tx.send(event.clone()).unwrap();
                    self.clear();
                } else if !self.keybinds.keys()
                    .any(|s| s.starts_with(&self.contents))
                {
                    self.clear();
                }
            },
            _ => match c {
                '\n' => self.run(),
                _ => self.contents.push(c),
            },
        }
    }

    pub fn clear(&mut self) {
        self.contents = "".to_string();
    }

    pub fn bind(&mut self, key: String, e: Event) {
        self.keybinds.insert(key, e);
    }

    pub fn run(&mut self) {
        match self.mode {
            Mode::Command => {
                let contents = self.contents.clone();
                let args: Vec<&str> = contents.split(" ").collect();

                let events = match command::parse(&args) {
                    Some(e) => match e {
                        Event::ToCommandLine(CommandLineEvent::Echo(_)) => vec![
                            spawn_mode_event(Mode::TUI),
                            e
                        ],
                        e => vec![
                            spawn_mode_event(Mode::TUI),
                            e.clone(),
                            spawn_respond_event(&e),
                        ],
                    },
                    None => vec![
                        spawn_mode_event(Mode::TUI),
                        spawn_invalid_event(&self.contents),
                    ],
                };

                for event in events {
                    self.tx.send(event).unwrap();
                }

            },
            Mode::Search => {
                self.tx.send(
                    Event::ToFocus(ComponentEvent::Search(self.contents.clone()))
                ).unwrap();
                self.tx.send(Event::ToCommandLine(CommandLineEvent::Mode(Mode::TUI))).unwrap();
            },
            _ => (),
        }
    }


    pub fn draw(&self) {
        let (w, h) = termion::terminal_size().unwrap();

        let prefix = match self.mode {
            Mode::Command => ":",
            Mode::Search => "/",
            _ => "",
        };

        match self.mode {
            Mode::Command | Mode::Search => print!(
                "{}{}{}{}",
               cursor::Goto(1, h),
               clear::CurrentLine,
               prefix,
               self.contents
            ),
            Mode::TUI if self.text.is_empty() => print!(
                "{}{}{}{}{}",
                cursor::Goto(1, h),
                clear::CurrentLine,
                self.statusline,
                cursor::Goto(w - self.contents.len() as u16, h),
                self.contents,
            ),
            Mode::TUI => print!(
                "{}{}{}{}{}",
                cursor::Goto(1, h),
                clear::CurrentLine,
                self.text,
                cursor::Goto(w - self.contents.len() as u16, h),
                self.contents,
            ),
        }
    }

    pub fn handle(&mut self, e: &CommandLineEvent, tx: mpsc::Sender<Event>) {
        match e {
            CommandLineEvent::Echo(s) => self.put_text(s.to_string()),
            CommandLineEvent::Mode(m) => self.mode(*m),
            CommandLineEvent::SbrcError(line, msg) => self.put_text(
                format!(
                    "sbrc: Invalid command at line {} '{}'",
                    line,
                    msg.to_string()
                )
            ),
            CommandLineEvent::Input(key) => match key {
                Key::Char(':') => tx.send(
                    Event::ToCommandLine(CommandLineEvent::Mode(Mode::Command))
                ).unwrap(),
                Key::Char('/') => tx.send(
                    Event::ToCommandLine(CommandLineEvent::Mode(Mode::Search))
                ).unwrap(),
                Key::Esc => tx.send(
                    Event::ToCommandLine(CommandLineEvent::Mode(Mode::TUI))
                ).unwrap(),
                Key::Backspace => if let Some(event) = self.back() {
                    tx.send(event).unwrap();
                },
                Key::Char(c) => self.add(*c),
                _ => (),
            },
            CommandLineEvent::SbrcNotFound => self.put_text(
                "Sbrc not found. :q to quit.".to_string()
            ),
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
        None => Err(())
    }
}

fn spawn_invalid_event(cmd: &str) -> Event {
    Event::ToCommandLine(CommandLineEvent::Echo(
        format!("Invalid Command '{}'", cmd)
    ))
}

fn spawn_respond_event(e: &Event) -> Event {
    Event::ToCommandLine(CommandLineEvent::Echo(format!("Ran: {:?}", e)))
}

fn spawn_mode_event(mode: Mode) -> Event {
    Event::ToCommandLine(CommandLineEvent::Mode(mode))
}
