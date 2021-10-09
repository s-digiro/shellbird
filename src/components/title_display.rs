use std::sync::mpsc;
use termion::{color, cursor};
use super::*;
use crate::event::Event;
use crate::color::Color;

pub struct TitleDisplay {
    name: String,
    contents: String,
    color: Color,
}

impl TitleDisplay {
    pub fn new(name: &str) -> TitleDisplay {
        TitleDisplay {
            name: name.to_string(),
            contents: String::new(),
            color: Color::Reset,
        }
    }
}

impl Component for TitleDisplay {
    fn name(&self) -> &str { &self.name }

    fn update(&mut self, event: &Event, _tx: mpsc::Sender<Event>) {
        match event {
            Event::NowPlaying(song) => {
                self.contents = match song {
                    Some(song) => match &song.title {
                        Some(title) => title.to_string(),
                        None => "<Empty>".to_string(),
                    },
                    None => "<Unavailable>".to_string(),
                };
            }
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, _h: u16) {
        let mut text = self.contents.clone();

        text.truncate(w.into());

        print!("{}{}{}{}",
            color::Fg(self.color),
            cursor::Goto(x, y),
            text,
            color::Fg(Color::Reset),
        );
    }
}
