use std::sync::mpsc;
use std::collections::HashSet;

use mpd::Song;

use unicode_truncate::Alignment;

use crate::components::{Component, Components, menu::{Menu, Parent}};
use crate::event::*;
use crate::GlobalState;
use crate::color::Color;

#[derive(Debug)]
#[derive(PartialEq)]
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
        parent: Option<String>
    ) -> Components {
        Components::TagMenu(
            TagMenu::new(
                name,
                color,
                focus_color,
                title,
                title_alignment,
                menu_alignment,
                tag,
                multitag_separator,
                parent,
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
        tag: &str,
        multitag_separator: Option<String>,
        parent: Option<String>
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

        Event::ToGlobal(GlobalEvent::TagMenuUpdated(self.name().to_string(), event_tracks))
    }

    pub fn set_menu_items(&mut self, library: &Vec<Song>) {
        self.menu.selection = 0;
        self.menu.items = vec!["<All>".to_string()];

        let items: Vec<String> = self.tracks.clone().iter()
            .filter(|id| library.get(**id) != None)
            .map(|id| match library.get(*id).unwrap().tags.get(&self.tag) {
                Some(val) => val.to_string(),
                None => "<Empty>".to_string(),
            }).collect();

        let mut final_items = Vec::new();

        if let Some(sep) = &self.multitag_separator {
            for item in items.iter() {
                if item.contains(sep) {
                    let mut new_tags = item.split(sep)
                        .map(|s| s.to_string())
                        .collect();

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
            self.tracks.iter()
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
                }).map(|u| *u)
                .collect()
        }
    }

    pub fn selected_tracks(&self, library: &Vec<Song>) -> Vec<Song> {
        self.tracks.iter()
            .filter(|id| None != library.get(**id))
            .filter(|id| match self.menu.selection() {
                Some(sel_tag) => match sel_tag.as_str() {
                    "<Empty>" => library.get(**id).unwrap().tags.get(&self.tag) == None,
                    sel_tag => match library.get(**id).unwrap().tags.get(&self.tag) {
                        Some(tag) => self.tag_is(tag, sel_tag),
                        None => false,
                    },
                },
                None => false,
            })
            .map(|id| library.get(*id).unwrap().clone())
            .collect()
    }
}

impl Component for TagMenu {
    fn name(&self) -> &str { &self.menu.name }

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
                            self.selected_tracks(&state.library))
                    )
                ).unwrap()
            },
            e => {
                self.menu.handle_focus(state, e, tx.clone());
                tx.send(self.spawn_update_event(&state.library)).unwrap();
            },
        }
    }

    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::StyleMenuUpdated(origin, styles) if self.parent.is(origin) => {
                if let Some(style_tree) = &state.style_tree {
                    let genres: HashSet<&str> = styles.iter()
                        .map(|id| style_tree.name(*id))
                        .collect();

                    let tracks = state.library.iter().enumerate()
                        .filter(|(_, song)| match &song.tags.get("Genre") {
                            Some(g) => genres.contains(g.as_str()),
                            None => false,
                        }).map(|(i, _)| i)
                        .collect();

                    self.tracks = tracks;

                    self.set_menu_items(&state.library);
                    tx.send(self.spawn_update_event(&state.library)).unwrap();
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
            },
            GlobalEvent::TagMenuUpdated(origin, tracks) if self.parent.is(origin) => {
                self.tracks = tracks.clone();

                self.set_menu_items(&state.library);
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            GlobalEvent::Database(tracks) if self.parent.is_none() => {
                self.tracks = (0..tracks.len()).collect();

                self.set_menu_items(&tracks);

                tx.send(self.spawn_update_event(&tracks)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            GlobalEvent::LostMpdConnection => {
                self.tracks = Vec::new();
                self.set_menu_items(&Vec::new());
                tx.send(self.spawn_update_event(&Vec::new())).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
