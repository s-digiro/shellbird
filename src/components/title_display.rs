use std::sync::mpsc;
use termion::{color, cursor};
use super::*;
use crate::color::Color;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TitleDisplay {
    name: String,
    contents: String,
    color: Color,
}

impl TitleDisplay {
    pub fn enumed(name: &str, color: Color) -> Components {
        Components::TitleDisplay(TitleDisplay::new(name, color))
    }

    pub fn new(name: &str, color: Color) -> TitleDisplay {
        TitleDisplay {
            name: name.to_string(),
            contents: String::new(),
            color,
        }
    }
}

impl Component for TitleDisplay {
    fn name(&self) -> &str { &self.name }

    fn handle_global(
        &mut self,
        _state: &GlobalState,
        e: &GlobalEvent,
        _tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::NowPlaying(song) => {
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

    fn draw(&self, x: u16, y: u16, w: u16, _h: u16, _focus: bool) {
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
