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
    align: Align,
}

impl TitleDisplay {
    pub fn enumed(name: &str, color: Color, align: Align) -> Components {
        Components::TitleDisplay(TitleDisplay::new(name, color, align))
    }

    pub fn new(name: &str, color: Color, align: Align) -> TitleDisplay {
        TitleDisplay {
            name: name.to_string(),
            contents: String::new(),
            color,
            align,
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
        let pad_left = self.align.pad_left(self.contents.len(), w);
        let pad_right = self.align.pad_right(self.contents.len(), w);

        let text = self.align.crop(&self.contents, w);

        print!("{}{}{}{}{}{}",
            color::Fg(self.color),
            cursor::Goto(x, y),
            pad_left,
            text,
            pad_right,
            color::Fg(Color::Reset),
        );
    }
}
