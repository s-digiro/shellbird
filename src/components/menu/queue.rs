use mpd::Song;
use std::sync::mpsc;

use termion::{color, cursor, style};
use crate::event::*;
use crate::GlobalState;
use crate::color::Color;
use crate::components::{Component, Components, menu::Menu};
use unicode_truncate::{UnicodeTruncateStr, Alignment};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Queue {
    name: String,
    tracks: Vec<Song>,
    menu: Menu,
    now_playing: Option<Song>,
}

impl Queue {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> Components {
        Components::Queue(
            Queue::new(
                name,
                color,
                focus_color,
                title,
                title_alignment,
                menu_alignment,
            )
        )
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> Queue {
        Queue {
            name: name.to_string(),
            tracks: Vec::new(),
            now_playing: None,
            menu: Menu {
                title,
                focus_color,
                color,
                selection: 0,
                items: Vec::new(),
                title_alignment,
                menu_alignment,
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

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            FocusEvent::Select => {
                if let Some(song) = self.tracks.get(self.menu.selection) {
                    tx.send(Event::ToMpd(
                        MpdEvent::PlayAt(song.clone())
                    )).unwrap()
                }
            },
            e => self.menu.handle_focus(state, e, tx.clone()),
        }
    }

    fn handle_global(
        &mut self,
        _state: &GlobalState,
        e: &GlobalEvent,
        _tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::NowPlaying(song) => self.set_now_playing(&song),
            GlobalEvent::Queue(q) => self.update_items(q),
            GlobalEvent::LostMpdConnection => {
                self.now_playing = None;
                self.update_items(&Vec::new());
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        let mut cur_y = y;

        let mut buffer = String::new();

        if let Some(title) = &self.menu.title {
            buffer.push_str(
                &format!(
                    "{}{}{}{}{}{}",
                    color::Fg(self.menu.color(focus)),
                    cursor::Goto(x, y),
                    title.unicode_pad(w as usize, self.menu.title_alignment, true),
                    cursor::Goto(x, y + 1),
                    "─".repeat(w as usize),
                    style::Reset,
                )
            );

            cur_y = cur_y + 2;
        }

        let mut i = self.menu.first_visible(h);
        for line in cur_y..(y + h) {
            if let Some(s) = self.menu.items.get(i) {
                let s = s.unicode_pad(w as usize, self.menu.menu_alignment, true);

                if self.menu.selection == i {
                    buffer.push_str(&format!("{}", style::Invert));
                }

                if let Some(np) = &self.now_playing {
                    if self.tracks.get(i) == Some(np) {
                        buffer.push_str(&format!("{}", style::Bold));
                    }
                }

                buffer.push_str(
                    &format!(
                        "{}{}{}{}",
                        color::Fg(self.menu.color(focus)),
                        cursor::Goto(x, line),
                        s,
                        style::Reset,
                    )
                );
            } else {
                buffer.push_str(
                    &format!(
                        "{}{}",
                        cursor::Goto(x, line),
                        " ".repeat(w as usize),
                    ),
                );
            }

            i = i + 1;
        }

        buffer.push_str(&format!("{}", style::Reset));

        print!("{}", buffer);
    }
}
