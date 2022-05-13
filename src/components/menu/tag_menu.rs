/* TUI Component for a menu of track tags
   Copyright (C) 2020-2022 Sean DiGirolamo

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

use std::collections::HashSet;
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
pub struct TagMenu {
    tag: String,
    parent: Parent,
    tracks: Vec<usize>,
    menu: Menu,
    multitag_separator: Option<String>,
}

impl TagMenu {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
        tag: &str,
        multitag_separator: Option<String>,
        parent: Option<String>,
    ) -> Components {
        Components::TagMenu(TagMenu::new(
            name,
            color,
            focus_color,
            title,
            title_alignment,
            menu_alignment,
            tag,
            multitag_separator,
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
        tag: &str,
        multitag_separator: Option<String>,
        parent: Option<String>,
    ) -> TagMenu {
        TagMenu {
            parent: Parent::new(parent),
            tag: tag.to_string(),
            tracks: Vec::new(),
            multitag_separator,
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

    pub fn tag_is(&self, tag_val: &str, target: &str) -> bool {
        if let Some(sep) = &self.multitag_separator {
            tag_val.split(sep).collect::<Vec<&str>>().contains(&target)
        } else {
            tag_val == target
        }
    }

    pub fn spawn_update_event(&self, library: &Vec<Song>) -> Event {
        let event_tracks = self.selection(library);

        Event::ToAllComponents(ComponentEvent::TagMenuUpdated(
            self.name().to_string(),
            event_tracks,
        ))
    }

    pub fn set_menu_items(&mut self, library: &Vec<Song>) {
        self.menu.selection = 0;
        self.menu.items = vec!["<All>".to_string()];

        let items: Vec<String> = self
            .tracks
            .clone()
            .iter()
            .filter(|id| library.get(**id) != None)
            .map(|id| match library.get(*id).unwrap().tags.get(&self.tag) {
                Some(val) => val.to_string(),
                None => "<Empty>".to_string(),
            })
            .collect();

        let mut final_items = Vec::new();

        if let Some(sep) = &self.multitag_separator {
            for item in items.iter() {
                if item.contains(sep) {
                    let mut new_tags =
                        item.split(sep).map(|s| s.to_string()).collect();

                    final_items.append(&mut new_tags);
                } else {
                    final_items.push(item.to_string());
                }
            }
        } else {
            final_items = items;
        }

        final_items.sort();
        final_items.dedup();

        self.menu.items.append(&mut final_items);
    }

    pub fn selection(&self, library: &Vec<Song>) -> Vec<usize> {
        if self.menu.selection == 0 {
            self.tracks.clone()
        } else {
            self.tracks
                .iter()
                .filter(|id| match library.get(**id) {
                    Some(song) => match self.menu.selection() {
                        Some(tag) => match tag.as_str() {
                            "<Empty>" => song.tags.get(&self.tag) == None,
                            tag => match song.tags.get(&self.tag) {
                                Some(t) => self.tag_is(t, tag),
                                None => false,
                            },
                        },
                        None => false,
                    },
                    None => false,
                })
                .map(|u| *u)
                .collect()
        }
    }

    pub fn selected_tracks(&self, library: &Vec<Song>) -> Vec<Song> {
        self.tracks
            .iter()
            .filter(|id| None != library.get(**id))
            .filter(|id| match self.menu.selection() {
                Some(sel_tag) => match sel_tag.as_str() {
                    "<Empty>" => {
                        library.get(**id).unwrap().tags.get(&self.tag) == None
                    },
                    sel_tag => {
                        match library.get(**id).unwrap().tags.get(&self.tag) {
                            Some(tag) => self.tag_is(tag, sel_tag),
                            None => false,
                        }
                    },
                },
                None => false,
            })
            .map(|id| library.get(*id).unwrap().clone())
            .collect()
    }
}

impl Component for TagMenu {
    fn name(&self) -> &str {
        &self.menu.name
    }

    fn handle(
        &mut self,
        state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>,
    ) {
        match e {
            ComponentEvent::Start => (),
            ComponentEvent::Next => {
                self.menu.next();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Prev => {
                self.menu.prev();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToTop => {
                self.menu.to_top();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToBottom => {
                self.menu.to_bottom();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoTo(i) => {
                self.menu.to(*i);
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Search(s) => {
                self.menu.search(s);
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::SearchPrev(s) => {
                self.menu.search_prev(s);
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Select => tx
                .send(Event::ToMpd(MpdEvent::AddToQueue(
                    self.selected_tracks(&state.library),
                )))
                .unwrap(),
            ComponentEvent::OpenTags => tx
                .send(Event::ToApp(AppEvent::TagUI(
                    self.selection(&state.library),
                )))
                .unwrap(),
            ComponentEvent::StyleMenuUpdated(origin, styles)
                if self.parent.is(origin) =>
            {
                if let Some(style_tree) = &state.style_tree {
                    let genres: HashSet<&str> =
                        styles.iter().map(|id| style_tree.name(*id)).collect();

                    let tracks = state
                        .library
                        .iter()
                        .enumerate()
                        .filter(|(_, song)| match &song.tags.get("Genre") {
                            Some(g) => genres.contains(g.as_str()),
                            None => false,
                        })
                        .map(|(i, _)| i)
                        .collect();

                    self.tracks = tracks;

                    self.set_menu_items(&state.library);
                    tx.send(self.spawn_update_event(&state.library)).unwrap();
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
            },
            ComponentEvent::TagMenuUpdated(origin, tracks)
                if self.parent.is(origin) =>
            {
                self.tracks = tracks.clone();

                self.set_menu_items(&state.library);
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Database if self.parent.is_none() => {
                let lib = &state.library;

                self.tracks = (0..lib.len()).collect();

                self.set_menu_items(lib);

                tx.send(self.spawn_update_event(lib)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::LostMpdConnection => {
                self.tracks = Vec::new();
                self.set_menu_items(&Vec::new());
                tx.send(self.spawn_update_event(&Vec::new())).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus == self.name());
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
