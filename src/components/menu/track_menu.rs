use std::sync::mpsc;

use mpd::Song;

use unicode_truncate::Alignment;

use crate::event::*;
use crate::color::Color;
use crate::GlobalState;
use crate::components::{Component, Components, menu::{Menu, Parent}};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TrackMenu {
    name: String,
    parent: Parent,
    menu: Menu,
    tracks: Vec<Song>,
}

impl TrackMenu {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
        parent: Option<String>,
    ) -> Components {
        Components::TrackMenu(
            TrackMenu::new(
                name,
                color,
                focus_color,
                title,
                title_alignment,
                menu_alignment,
                parent
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
        parent: Option<String>
    ) -> TrackMenu {
        TrackMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            tracks: Vec::new(),
            menu: Menu {
                title,
                title_alignment,
                menu_alignment,
                color,
                focus_color,
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
        _tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::LostMpdConnection => {
                self.tracks = Vec::new();
                self.update_menu_items();
            },
            GlobalEvent::PlaylistMenuUpdated(name, pl) if self.parent.is(name) => match pl {
                Some(pl) => {
                    self.tracks = pl.tracks.clone();
                    self.update_menu_items();
                },
                None => (),
            },
            GlobalEvent::TagMenuUpdated(name, tracks) if self.parent.is(name) => {
                self.tracks = tracks.iter()
                    .filter(|id| state.library.get(**id) != None)
                    .map(|id| state.library.get(*id).unwrap().clone())
                    .collect();

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

                    let tracks = {
                        let mut ret = Vec::new();
                        for genre in genres {
                            if let Some(tracks) = tree.tracks(&Some(genre)) {
                                ret.append(&mut tracks.clone())
                            }
                        }
                        ret
                    };

                    self.tracks = tracks;
                    self.update_menu_items();
                }
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
