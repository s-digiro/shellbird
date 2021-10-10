mod command;

use std::collections::HashMap;
use std::sync::mpsc;
use termion::{cursor, clear};

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
    pub fn new(keybinds: HashMap<String, Event>, tx: mpsc::Sender<Event>) -> CommandLine {
        CommandLine {
            contents: String::new(),
            statusline: String::new(),
            text: String::new(),
            mode: Mode::TUI,
            keybinds,
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
            Some(Event::ToApp(AppEvent::Mode(Mode::TUI)))
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

                match command::parse(&args) {
                    Some(e) => match e {
                        Event::ToApp(AppEvent::Echo(_)) => {
                            self.tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap();
                            self.tx.send(e).unwrap();
                        },
                        e =>{
                            let msg = format!("Ran: {:?}", e);
                            self.tx.send(e).unwrap();
                            self.tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap();
                            self.tx.send(self.respond(msg)).unwrap();
                        }
                    },
                    None => {
                        self.tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap();
                        self.tx.send(self.invalid()).unwrap();
                    },
                }

            },
            Mode::Search => {
                self.tx.send(
                    Event::ToFocus(FocusEvent::Search(self.contents.clone()))
                ).unwrap();
                self.tx.send(Event::ToApp(AppEvent::Mode(Mode::TUI))).unwrap();
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
