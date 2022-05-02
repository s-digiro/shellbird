/* TUI Component for Menu of tracks
   Copyright (C) 2020-2021 Sean DiGirolamo

This file is part of Shellbird.

Shellbird is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation; either version 3, or (at your option) any
later version.

Shellbird is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Shellbird; see the file COPYING.  If not see
<http://www.gnu.org/licenses/>.  */

use std::sync::mpsc;

use mpd::Song;

use unicode_truncate::Alignment;

use crate::color::Color;
use crate::components::{
    menu::{Menu, Parent},
    Component, Components,
};
use crate::event::*;
use crate::GlobalState;

#[derive(Debug, PartialEq)]
pub struct TrackMenu {
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
        Components::TrackMenu(TrackMenu::new(
            name,
            color,
            focus_color,
            title,
            title_alignment,
            menu_alignment,
            parent,
        ))
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
        parent: Option<String>,
    ) -> TrackMenu {
        TrackMenu {
            parent: Parent::new(parent),
            tracks: Vec::new(),
            menu: Menu {
                name: name.to_string(),
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
        self.menu.items = self
            .tracks
            .iter()
            .map(|s| match &s.title {
                Some(title) => title.to_string(),
                None => "<Empty>".to_string(),
            })
            .collect();
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
    fn name(&self) -> &str {
        &self.menu.name
    }

    fn handle(&mut self, state: &GlobalState, e: &ComponentEvent, tx: mpsc::Sender<Event>) {
        match e {
            ComponentEvent::OpenTags =>
                tx.send(
                    Event::ToApp(AppEvent::TagUI(self.selected_tracks()))
                ).unwrap(),
            ComponentEvent::Select => {
                tx.send(Event::ToMpd(MpdEvent::AddToQueue(self.selected_tracks())))
                    .unwrap();
            }
            ComponentEvent::Next => {
                self.menu.next();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::Prev => {
                self.menu.prev();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::GoToTop => {
                self.menu.to_top();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::GoToBottom => {
                self.menu.to_bottom();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::GoTo(i) => {
                self.menu.to(*i);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::Search(s) => {
                self.menu.search(s);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::SearchPrev(s) => {
                self.menu.search_prev(s);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name())
            }
            ComponentEvent::Start => {
                if let Some(track) = self.selected_tracks().first() {
                    tx.send(Event::ToMpd(MpdEvent::PlayAt(track.clone())))
                        .unwrap();
                }
            }
            ComponentEvent::LostMpdConnection => {
                self.tracks = Vec::new();
                self.update_menu_items();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::PlaylistMenuUpdated(name, pl) if self.parent.is(name) => match pl {
                Some(pl) => {
                    self.tracks = pl.tracks.clone();
                    self.update_menu_items();
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
                None => (),
            },
            ComponentEvent::TagMenuUpdated(name, tracks) if self.parent.is(name) => {
                self.tracks = tracks
                    .iter()
                    .filter(|id| state.library.get(**id) != None)
                    .map(|id| state.library.get(*id).unwrap().clone())
                    .collect();

                self.update_menu_items();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            }
            ComponentEvent::StyleMenuUpdated(name, styles) if self.parent.is(name) => {
                if let Some(tree) = &state.style_tree {
                    let genres = {
                        let mut leaves = Vec::new();

                        for style in styles {
                            leaves.append(
                                &mut tree
                                    .leaf_names(*style)
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect(),
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
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
            }
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
