mod command;

use std::fmt;
use std::collections::HashMap;
use std::sync::mpsc;
use termion::{cursor, clear};

use crate::event::*;
use crate::mode::Mode;

pub struct CommandLine {
    contents: String,
    mode: Mode,
    keybinds: HashMap<String, Event>,
    tx: mpsc::Sender<Event>,
}

impl CommandLine {
    pub fn new(keybinds: HashMap<String, Event>, tx: mpsc::Sender<Event>) -> CommandLine {
        CommandLine {
            contents: String::new(),
            mode: Mode::TUI,
            keybinds,
            tx,
        }
    }

    pub fn mode(&mut self, mode: Mode) {
        self.clear();
        self.mode = mode;
    }

    pub fn back(&mut self) -> Option<Event> {
        if self.contents.is_empty() {
            Some(Event::ToApp(AppEvent::Mode(Mode::TUI)))
        } else {
            self.contents.pop();
            None
        }
    }

    pub fn add(&mut self, c: char) {
        match self.mode {
            Mode::TUI => {
                self.contents.push(c);

                if let Some(event) = self.keybinds.get(&self.contents) {
                    self.tx.send(event.clone()).unwrap();
                    self.clear();
                } else if !self.keybinds.keys()
                    .any(|s| s.starts_with(&self.contents)) 
                {
                    self.tx.send(
                        Event::ToApp(
                            AppEvent::InvalidCommand(self.contents.clone())
                        )
                    ).unwrap();
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
        self.contents = String::new();
    }

    pub fn bind(&mut self, key: String, e: Event) {
        self.keybinds.insert(key, e);
    }

    pub fn run(&mut self) {
        match self.mode {
            Mode::Command => {
                let contents = self.contents.clone();
                let args: Vec<&str> = contents.split(" ").collect();

                match command::parse(&args) {
                    Some(e) => {
                        let msg = format!("Ran: {:?}", e);
                        self.tx.send(e).unwrap();
                        self.tx.send(self.respond(msg)).unwrap();
                    },
                    None => self.tx.send(self.invalid()).unwrap(),
                }

                self.clear();
                self.tx.send(self.reset()).unwrap();
            },
            Mode::Search => {
                self.tx.send(
                    Event::ToFocus(FocusEvent::Search(self.contents.clone()))
                ).unwrap();
                self.clear();
                self.tx.send(self.reset()).unwrap();
            },
            _ => (),
        }
    }

    fn invalid(&self) -> Event {
        Event::ToApp(AppEvent::InvalidCommand(self.contents.clone()))
    }

    fn respond(&self, s: String) -> Event {
        Event::ToApp(AppEvent::CommandResponse(s))
    }

    fn reset(&self) -> Event {
        Event::ToApp(AppEvent::Mode(Mode::TUI))
    }

}

impl fmt::Display for CommandLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (_, h) = termion::terminal_size().unwrap();

        let prefix = match self.mode {
            Mode::Command => ":",
            Mode::Search => "/",
            _ => "",
        };

        write!(f, "{}{}{}{}",
               cursor::Goto(0, h),
               clear::CurrentLine,
               prefix,
               self.contents,
        )
    }
}
