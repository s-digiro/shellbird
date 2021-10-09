use mpd::Song;
use std::sync::mpsc;

use termion::{color, cursor, style};
use crate::event::*;
use crate::color::Color;
use crate::components::{Component, menu::Menu};

pub struct Queue {
    name: String,
    tracks: Vec<Song>,
    menu: Menu,
    color: Color,
    now_playing: Option<Song>,
}

impl Queue {
    pub fn new(name: &str) -> Queue {
        Queue {
            name: name.to_string(),
            tracks: Vec::new(),
            color: Color::Reset,
            now_playing: None,
            menu: Menu {
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    fn set_now_playing(&mut self, target: &Option<Song>) {
        match target {
            Some(target) => self.now_playing = Some(target.clone()),
            None => self.now_playing = None,
        }
    }

    fn update_items(&mut self, tracks: &Vec<Song>) {
        self.tracks = tracks.clone();
        self.update_menu_items();
    }

    fn update_menu_items(&mut self) {
        self.menu.items = self.tracks.iter()
            .map(|s| match &s.title {
                Some(title) => title.to_string(),
                None => "<Empty>".to_string(),
            }).collect();
    }
}

impl Component for Queue {
    fn name(&self) -> &str { &self.name }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        match e {
            FocusEvent::Select => {
                if let Some(song) = self.tracks.get(self.menu.selection) {
                    tx.send(Event::ToMpd(
                        MpdEvent::PlayAt(song.clone())
                    )).unwrap()
                }
            },
            e => self.menu.handle_focus(e, tx.clone()),
        }
    }

    fn handle_global(&mut self, e: &GlobalEvent, _tx: mpsc::Sender<Event>) {
        match e {
            GlobalEvent::NowPlaying(song) => self.set_now_playing(&song),
            GlobalEvent::Queue(q) => self.update_items(q),
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        print!("{}", color::Fg(self.color));

        let first_visible = self.menu.first_visible(h);

        let mut line = y;

        for (i, track) in self.tracks.iter().enumerate().skip(first_visible) {
            let mut name = match &track.title {
                Some(title) => title.to_string(),
                None => "<Empty>".to_string(),
            };

            name.truncate(w as usize);

            if self.menu.selection == i {
                print!("{}", style::Invert);
            }
            if let Some(np) = &self.now_playing {
                if track == np {
                    print!("{}", style::Bold);
                }
            }
            print!("{}{}{}{}",
                cursor::Goto(x, line),
                name,
                " ".repeat(w as usize - name.len()),
                style::Reset,
            );

            line += 1;

            if line >= y + h {
                break;
            }
        }
    }
}
