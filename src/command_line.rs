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

    pub fn parse(&mut self) -> Option<Event> {
        match self.mode {
            Mode::Command => {
                let contents = self.contents.clone();
                let args: Vec<&str> = contents.split(" ").collect();

                let invalid = Some(self.invalid());

                match args.get(0) {
                    Some(arg) => match *arg {
                        "q" => Some(Event::ToApp(AppEvent::Quit)),
                        "pause" => Some(Event::ToMpd(MpdEvent::TogglePause)),
                        "bind" =>Some(self.parse_bind(&args)),
                        _ => invalid
                    },
                    _ => invalid
                }
            },
            Mode::Search => Some(Event::ToFocus(FocusEvent::Search(self.contents.clone()))),
            _ => None,
        }
    }

    fn invalid(&self) -> Event {
        Event::ToApp(AppEvent::InvalidCommand(self.contents.clone()))
    }

    fn respond(&self, s: String) -> Event {
        Event::ToApp(AppEvent::CommandResponse(s))
    }

    fn parse_bind(&mut self, args: &Vec<&str>) -> Event {
        match args.get(1) {
            Some(s) => {
                eprintln!("Got {}", s);
                match args.get(2) {
                    Some(a) => match a.to_string().to_lowercase().as_str() {
                        "toapp" => self.parse_bind_app(s, args),
                        //"toscreen" => self.parse_bind_screen(args),
                        //"toglobal" => self.parse_bind_global(args),
                        //"tofocus" => self.parse_bind_focus(args),
                        //"tompd" => self.parse_bind_mpd(args),
                        _ => self.invalid()
                    },
                    _ => self.invalid()
                }
            }
            _ => self.invalid()
        }
    }

    fn parse_bind_app(&mut self, seq: &str, args: &Vec<&str>) -> Event {
        eprintln!("Bind app");
        eprintln!("{:?}", args.get(3));
        match args.get(3) {
            Some(a) => match a.to_string().to_lowercase().as_str() {
                "switchscreen" => match args.get(4) {
                    Some(a) => match a.parse::<usize>() {
                        Ok(num) => {
                            self.keybinds.insert(seq.to_string(), Event::ToApp(AppEvent::SwitchScreen(num)));
                            self.respond(format!("Bound {} to switchscreen {}", seq, num))
                        },
                        _ => self.invalid(),
                    },
                    _ => self.invalid(),
                },
                "quit" => {
                    self.keybinds.insert(seq.to_string(), Event::ToApp(AppEvent::Quit));
                    self.respond(format!("Bound {} to quit", seq))
                },
                _ => self.invalid(),
            },
            _ => self.invalid(),
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
