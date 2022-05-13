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
use termion::{clear, color, cursor, event::Key};

use crate::color::Color;
use crate::event::*;

#[derive(Debug)]
enum Mode {
    TUI {
        text: String,
        keys: Vec<Key>,
    },

    Command {
        text: String,
    },

    Search {
        text: String,
    },

    GetText {
        prompt: String,
        text: String,
    },

    Confirm {
        prompt: String,
        on_yes: Option<ConfirmableEvent>,
        on_no: Option<ConfirmableEvent>,
        is_default_yes: bool,
    },
}

pub struct CommandLine {
    mode: Mode,
    statusline: String,
    keybinds: HashMap<Vec<Key>, Event>,
    tx: mpsc::Sender<Event>,

    color: Color,

    last_search: Option<String>,
    volume: i8,
}

impl CommandLine {
    pub fn new(tx: mpsc::Sender<Event>) -> CommandLine {
        CommandLine {
            mode: Mode::TUI {
                text: String::new(),
                keys: Vec::new(),
            },
            statusline: "[----] Vol: %".into(),
            keybinds: HashMap::new(),
            tx,

            color: Color::Reset,

            last_search: None,
            volume: -1,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn echo(&mut self, new_text: String) {
        if let Mode::TUI { ref mut text, .. } = &mut self.mode {
            *text = new_text;
        }
    }

    pub fn tui_mode(&mut self) {
        self.mode = Mode::TUI {
            text: String::new(),
            keys: Vec::new(),
        };
    }

    pub fn command_mode(&mut self) {
        self.mode = Mode::Command {
            text: String::new(),
        };
    }

    pub fn search_mode(&mut self) {
        self.mode = Mode::Search {
            text: String::new(),
        };
    }

    pub fn get_text_mode(
        &mut self,
        prompt: String,
        placeholder: Option<String>,
    ) {
        self.mode = Mode::GetText {
            prompt,
            text: placeholder.unwrap_or(String::new()),
        };
    }

    pub fn confirm_mode(
        &mut self,
        prompt: String,
        on_yes: Option<ConfirmableEvent>,
        on_no: Option<ConfirmableEvent>,
        is_default_yes: bool,
    ) {
        self.mode = Mode::Confirm {
            prompt,
            on_yes,
            on_no,
            is_default_yes,
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
        match &mut self.mode {
            Mode::TUI { keys, text } => match key {
                Key::Char(':') => self.command_mode(),
                Key::Char('/') => self.search_mode(),
                Key::Esc => {
                    text.clear();
                    keys.clear();
                },
                key => {
                    text.clear();

                    keys.push(key.clone());

                    if let Some(event) = self.keybinds.get(keys) {
                        self.tx.send(event.clone()).unwrap();

                        keys.clear();
                    } else {
                        let has_match = self.keybinds.keys().any(|kb| {
                            keys.iter().zip(kb.iter()).all(|(a, b)| a == b)
                        });

                        if !has_match {
                            keys.clear();
                        }
                    }
                },
            },
            Mode::GetText { text, .. } => match key {
                Key::Char('\n') => self.run(),
                Key::Esc => self.tui_mode(),
                Key::Backspace => {
                    if !text.is_empty() {
                        text.pop();
                    }
                },
                Key::Char(c) => text.push(*c),
                _ => (),
            },
            Mode::Search { text } | Mode::Command { text } => match key {
                Key::Char('\n') => self.run(),
                Key::Esc => self.tui_mode(),
                Key::Backspace => {
                    if text.is_empty() {
                        self.tui_mode();
                    } else {
                        text.pop();
                    }
                },
                Key::Char(c) => text.push(*c),
                _ => (),
            },

            Mode::Confirm {
                on_yes,
                on_no,
                is_default_yes,
                ..
            } => match key {
                Key::Char('\n') => {
                    match (&on_yes, &on_no, is_default_yes) {
                        (Some(_), _, true) => self
                            .tx
                            .send(on_yes.take().unwrap().to_event())
                            .unwrap(),
                        (_, Some(_), false) => self
                            .tx
                            .send(on_no.take().unwrap().to_event())
                            .unwrap(),
                        _ => (),
                    }

                    self.tui_mode();
                },

                Key::Esc => self.tui_mode(),

                Key::Char('y') | Key::Char('Y') => {
                    if let Some(_) = on_yes {
                        self.tx
                            .send(on_yes.take().unwrap().to_event())
                            .unwrap();
                    }

                    self.tui_mode();
                },

                Key::Char('n') | Key::Char('N') => {
                    if let Some(_) = on_no {
                        self.tx.send(on_no.take().unwrap().to_event()).unwrap();
                    }

                    self.tui_mode();
                },
                _ => (),
            },
        }
    }

    pub fn bind(&mut self, keys: Vec<Key>, e: Event) {
        self.keybinds.insert(keys, e);
    }

    pub fn run(&mut self) {
        let mode = mem::replace(
            &mut self.mode,
            Mode::TUI {
                text: String::new(),
                keys: Vec::new(),
            },
        );

        match mode {
            Mode::Command { text } => {
                match command::parse(&text) {
                    Some(e) => match e {
                        Event::ToCommandLine(CommandLineEvent::Echo(s)) => {
                            self.echo(s.to_owned())
                        },
                        e => {
                            self.tx.send(e.clone()).unwrap();
                            self.tx
                                .send(Event::ToCommandLine(
                                    CommandLineEvent::Echo(format!(
                                        "Ran: {:?}",
                                        e
                                    )),
                                ))
                                .unwrap();
                        },
                    },
                    None => self
                        .tx
                        .send(Event::ToCommandLine(CommandLineEvent::Echo(
                            format!("Invalid Command '{}'", text),
                        )))
                        .unwrap(),
                };
            },

            Mode::Search { text } => {
                self.last_search = Some(text.clone());

                self.tx
                    .send(Event::ToFocus(ComponentEvent::Search(text)))
                    .unwrap();
            },

            Mode::GetText { text, .. } => self
                .tx
                .send(Event::ToFocus(ComponentEvent::ReturnText(text)))
                .unwrap(),

            Mode::Confirm { .. } => {
                panic!("Invalid State: CommandLine called run while in Mode::Confirm")
            },

            Mode::TUI { .. } => panic!(
                "Invalid State: CommandLine called run while in Mode::TUI"
            ),
        }
    }

    pub fn draw(&self) {
        let (w, h) = termion::terminal_size().unwrap();

        print!(
            "{}{}{}",
            color::Fg(self.color),
            cursor::Goto(1, h),
            clear::CurrentLine
        );

        match &self.mode {
            Mode::TUI { text, .. } => print!("{}", text),
            Mode::Command { text } => print!(":{}🭰", text),
            Mode::Search { text } => print!("/{}🭰", text),
            Mode::GetText { prompt, text } => print!("{}: {}🭰", prompt, text),
            Mode::Confirm {
                prompt,
                is_default_yes,
                ..
            } => print!(
                "{} ({})",
                prompt,
                if *is_default_yes { "Y/n" } else { "y/N" }
            ),
        }

        print!(
            "{}{}",
            cursor::Goto(w - (self.statusline.len() as u16), h),
            self.statusline
        );
    }

    pub fn handle(&mut self, e: &CommandLineEvent, _tx: mpsc::Sender<Event>) {
        match e {
            CommandLineEvent::Echo(s) => self.echo(s.to_owned()),
            CommandLineEvent::SetColor(c) => self.color = *c,
            CommandLineEvent::RequestText(prompt, placeholder) => {
                self.get_text_mode(prompt.to_owned(), placeholder.to_owned())
            },
            CommandLineEvent::NextSearch => self.next_search(),
            CommandLineEvent::PrevSearch => self.prev_search(),
            CommandLineEvent::SbrcError(line, msg) => self.echo(format!(
                "sbrc: Invalid command at line {} '{}'",
                line, msg
            )),
            CommandLineEvent::Input(key) => self.input(&key),
            CommandLineEvent::SbrcNotFound => {
                self.echo("Sbrc not found. :q to quit.".to_owned())
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
    match command::parse(cmd) {
        Some(event) => {
            tx.send(event).unwrap();
            Ok(())
        },
        None => Err(()),
    }
}
