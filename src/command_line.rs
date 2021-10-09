use std::fmt;
use std::collections::HashMap;
use termion::{cursor, clear};
use crate::event::*;
use crate::mode::Mode;

pub struct CommandLine {
    contents: String,
    mode: Mode,
    keybinds: HashMap<String, Event>,
}

impl CommandLine {
    pub fn new(keybinds: HashMap<String, Event>) -> CommandLine {
        CommandLine {
            contents: String::new(),
            mode: Mode::TUI,
            keybinds,
        }
    }

    pub fn mode(&mut self, mode: Mode) {
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

    pub fn add(&mut self, c: char) -> Option<Event> {
        match self.mode {
            Mode::TUI => {
                self.contents.push(c);

                if let Some(event) = self.keybinds.get(&self.contents) {
                    Some(event.clone())
                } else if self.keybinds.keys().any(|s| s.starts_with(&self.contents)) {
                    None
                } else {
                    Some(Event::ToApp(AppEvent::InvalidCommand(self.contents.clone())))
                }
            },
            _ => match c {
                '\n' => self.parse(),
                _ => {
                    self.contents.push(c);
                    None
                },
            },
        }
    }

    pub fn clear(&mut self) {
        self.contents = String::new();
    }

    pub fn parse(&self) -> Option<Event> {
        match self.mode {
            Mode::Command => {
                let args: Vec<&str> = self.contents.split(" ").collect();

                let invalid = Event::ToApp(AppEvent::InvalidCommand(self.contents.clone()));

                Some(
                    match args.get(0) {
                        Some(arg) => match *arg {
                            "pause" => Event::ToMpd(MpdEvent::TogglePause),
                            _ => invalid
                        },
                        _ => invalid
                    }
                )
            },
            Mode::Search => Some(Event::ToFocus(FocusEvent::Search(self.contents.clone()))),
            _ => None,
        }
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
