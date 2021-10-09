use std::sync::mpsc;
use mpd::Song;
use crate::components::{Component, menu::{Menu, Parent}};
use crate::event::{ComponentRequest, MpdRequest, Event};

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

        Event::TagMenuUpdated(self.name.clone(), event_tracks)
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

    fn handle_request(&mut self, request: &ComponentRequest, tx: mpsc::Sender<Event>) {
        match request {
            ComponentRequest::Select => {
                tx.send(
                    Event::MpdRequest(MpdRequest::AddToQueue(
                            self.selected_tracks()
                    ))
                ).unwrap()
            },
            request => {
                self.menu.handle_request(request, tx.clone());
                tx.send(self.spawn_update_event()).unwrap();
            },
        }
    }

    fn update(&mut self, event: &Event, tx: mpsc::Sender<Event>) {
        match event {
            Event::TagMenuUpdated(origin, tracks) if self.parent.is(origin) => {
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
            Event::Database(tracks) if self.parent.is_none() => {
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
