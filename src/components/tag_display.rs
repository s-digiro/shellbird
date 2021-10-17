use super::*;
use std::sync::mpsc;
use termion::{cursor, color};
use crate::color::Color;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TagDisplay {
    name: String,
    tag: String,
    contents: String,
    color: Color,
}

impl TagDisplay {
    pub fn enumed(name: &str, color: Color, tag: &str) -> Components {
        Components::TagDisplay(TagDisplay::new(name, color, tag))
    }

    pub fn new(name: &str, color: Color, tag: &str) -> TagDisplay {
        TagDisplay {
            name: name.to_string(),
            tag: tag.to_string(),
            contents: String::new(),
            color,
        }
    }
}

impl Component for TagDisplay {
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
                    Some(song) => match song.tags.get(&self.tag) {
                        Some(title) => title.to_string(),
                        None => "<Empty>".to_string(),
                    },
                    None => "<Unavailable>".to_string(),
                }
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, _h: u16) {
        let mut text = self.contents.clone();

        text.truncate(w as usize);

        print!("{}{}{}{}",
               color::Fg(self.color),
               cursor::Goto(x, y),
               text,
               color::Fg(color::Reset),
        );
    }
}
