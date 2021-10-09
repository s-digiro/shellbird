use std::sync::mpsc;
use mpd::Song;
use crate::event::{ComponentRequest, Event};
use crate::components::{Component, menu::{Menu, Parent}};

pub struct TrackMenu {
    name: String,
    parent: Parent,
    menu: Menu,
    tracks: Vec<Song>,
}

impl TrackMenu {
    pub fn new(name: &str, parent: Option<String>) -> TrackMenu {
        TrackMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            tracks: Vec::new(),
            menu: Menu {
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    fn update_menu_items(&mut self) {
        self.menu.items = self.tracks.iter()
            .map(|s| match &s.title {
                Some(title) => title.to_string(),
                None => "<Empty>".to_string(),
            }).collect();
    }
}

impl Component for TrackMenu {
    fn name(&self) -> &str { &self.name }

    fn handle_request(&mut self, request: &ComponentRequest, tx: mpsc::Sender<Event>) {
        match request {
            request => self.menu.handle_request(request, tx.clone()),
        }
    }

    fn update(&mut self, event: &Event, _tx: mpsc::Sender<Event>) {
        match event {
            Event::PlaylistMenuUpdated(name, pl) if self.parent.is(name) => match pl {
                Some(pl) => {
                    self.tracks = pl.tracks.clone();
                    self.update_menu_items();
                },
                None => (),
            },
            Event::TagMenuUpdated(name, tracks) if self.parent.is(name) => {
                self.tracks = tracks.clone();
                self.update_menu_items();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        self.menu.draw(x, y, w, h);
    }
}
