use std::sync::mpsc;
use mpd::Song;
use crate::event::*;
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

    fn selected_tracks(&self) -> Vec<Song> {
        if let Some(track) = self.tracks.get(self.menu.selection) {
            vec![track.clone()]
        } else {
            Vec::new()
        }
    }
}

impl Component for TrackMenu {
    fn name(&self) -> &str { &self.name }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        match e {
            FocusEvent::Select => {
                tx.send(
                    Event::ToMpd(MpdEvent::AddToQueue(
                        self.selected_tracks()
                    ))
                ).unwrap();
            },
            e => self.menu.handle_focus(e, tx.clone()),
        }
    }

    fn handle_global(&mut self, e: &GlobalEvent, tx: mpsc::Sender<Event>) {
        match e {
            GlobalEvent::PlaylistMenuUpdated(name, pl) if self.parent.is(name) => match pl {
                Some(pl) => {
                    self.tracks = pl.tracks.clone();
                    self.update_menu_items();
                },
                None => (),
            },
            GlobalEvent::TagMenuUpdated(name, tracks) if self.parent.is(name) => {
                self.tracks = tracks.clone();
                self.update_menu_items();
            },
            GlobalEvent::StyleMenuUpdated(name, styles) if self.parent.is(name) => {
                let mut genres = Vec::new();
                for style in styles {
                    for genre in style.leaves() {
                        genres.push(genre);
                    }
                }

                tx.send(Event::ToMpd(MpdEvent::GetTracksFromGenres(
                    self.name.to_string(),
                    genres,
                ))).unwrap();
            },
            GlobalEvent::ReturnTracksTo(name, tracks) if self.name == name.to_string() => {
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
