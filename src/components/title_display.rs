use std::sync::mpsc;
use termion::{color, cursor};
use super::*;
use crate::color::Color;
use unicode_truncate::{UnicodeTruncateStr, Alignment};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TitleDisplay {
    name: String,
    contents: String,
    color: Color,
    alignment: Alignment,
}

impl TitleDisplay {
    pub fn enumed(name: &str, color: Color, alignment: Alignment) -> Components {
        Components::TitleDisplay(TitleDisplay::new(name, color, alignment))
    }

    pub fn new(name: &str, color: Color, alignment: Alignment) -> TitleDisplay {
        TitleDisplay {
            name: name.to_string(),
            contents: String::new(),
            color,
            alignment,
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
            GlobalEvent::LostMpdConnection => {
                self.contents = "<Unavailable>".to_string();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, _h: u16, _focus: bool) {
        print!("{}{}{}{}",
            color::Fg(self.color),
            cursor::Goto(x, y),
            self.contents.unicode_pad(w as usize, self.alignment, true),
            color::Fg(Color::Reset),
        );
    }
}
