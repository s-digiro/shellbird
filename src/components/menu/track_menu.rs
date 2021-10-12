use std::sync::mpsc;
use mpd::Song;
use crate::event::*;
use crate::GlobalState;
use crate::components::{Component, menu::{Menu, Parent}};

const STYLE_MENU_UPDATE_DELAY: u64 = 500;

pub struct TrackMenu {
    name: String,
    parent: Parent,
    menu: Menu,
    tracks: Vec<Song>,

    last_timestamp: std::time::SystemTime,
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

            last_timestamp: std::time::SystemTime::now(),
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

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            FocusEvent::Select => {
                tx.send(
                    Event::ToMpd(MpdEvent::AddToQueue(
                        self.selected_tracks()
                    ))
                ).unwrap();
            },
            e => self.menu.handle_focus(state, e, tx.clone()),
        }
    }

    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
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
                if let Some(tree) = &state.style_tree {
                    let genres = {
                        let mut leaves = Vec::new();

                        for style in styles {
                            leaves.append(
                                &mut tree.leaf_names(*style).iter()
                                    .map(|s| s.to_string())
                                    .collect()
                            );
                        }

                        leaves
                    };

                    let timestamp = std::time::SystemTime::now();

                    self.last_timestamp = timestamp.clone();

                    let event = Event::ToGlobal(GlobalEvent::PostponeMpd(
                        self.name.to_string(),
                        std::time::Duration::from_millis(STYLE_MENU_UPDATE_DELAY),
                        timestamp,
                        MpdEvent::GetTracksFromGenres(
                            self.name.to_string(),
                            genres,
                        ),
                    ));

                    tx.send(event).unwrap();
                }
            },
            GlobalEvent::PostponeMpd(name, wait_amount, timestamp, mpde)
            if self.name == name.to_string() => {
                let timestamp = timestamp.clone();
                let wait_amount = wait_amount.clone();
                if timestamp == self.last_timestamp {
                    if timestamp + wait_amount <= std::time::SystemTime::now() {
                        tx.send(Event::ToMpd(mpde.clone())).unwrap();
                    } else {
                        tx.send(Event::ToGlobal(e.clone())).unwrap();
                    }
                }
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
