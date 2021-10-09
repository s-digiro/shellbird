use std::sync::mpsc;
use mpd::Song;
use crate::components::{Component, menu::{Menu, Parent}};
use crate::event::*;

pub struct TagMenu {
    name: String,
    tag: String,
    parent: Parent,
    tracks: Vec<Song>,
    menu: Menu,
}

impl TagMenu {
    pub fn new(name: &str, tag: &str, parent: Option<String>) -> TagMenu {
        TagMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            tag: tag.to_string(),
            tracks: Vec::new(),
            menu: Menu {
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    pub fn spawn_update_event(&self) -> Event {
        let event_tracks = self.selected_tracks();

        Event::ToGlobal(GlobalEvent::TagMenuUpdated(self.name.clone(), event_tracks))
    }

    pub fn selected_tracks(&self) -> Vec<Song> {
        self.tracks.iter()
            .filter(|song| song.tags.get(&self.tag) == self.menu.selection())
            .map(|song| song.clone())
            .collect()
    }
}

impl Component for TagMenu {
    fn name(&self) -> &str { &self.name }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        match e {
            FocusEvent::Select => {
                tx.send(
                    Event::ToMpd(MpdEvent::AddToQueue(
                            self.selected_tracks()
                    ))
                ).unwrap()
            },
            e => {
                self.menu.handle_focus(e, tx.clone());
                tx.send(self.spawn_update_event()).unwrap();
            },
        }
    }

    fn handle_global(&mut self, e: &GlobalEvent, tx: mpsc::Sender<Event>) {
        match e {
            GlobalEvent::TagMenuUpdated(origin, tracks) if self.parent.is(origin) => {
                self.tracks = tracks.clone();

                self.menu.items = tracks.clone().iter()
                    .map(|song| match song.tags.get(&self.tag) {
                        Some(val) => val.to_string(),
                        None => "<Empty>".to_string(),
                    }).collect();

                self.menu.items.sort();
                self.menu.items.dedup();

                tx.send(self.spawn_update_event()).unwrap();
            },
            GlobalEvent::Database(tracks) if self.parent.is_none() => {
                self.tracks = tracks.clone();

                self.menu.items = tracks.clone().iter()
                    .map(|song| match song.tags.get(&self.tag) {
                        Some(val) => val.to_string(),
                        None => "<Empty>".to_string(),
                    }).collect();

                self.menu.items.sort();
                self.menu.items.dedup();

                tx.send(self.spawn_update_event()).unwrap()
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        self.menu.draw(x, y, w, h);
    }
}
