use std::sync::mpsc;
use std::collections::HashSet;
use mpd::Song;
use crate::components::{Component, Components, menu::{Menu, Parent}};
use crate::event::*;
use crate::GlobalState;
use crate::color::Color;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TagMenu {
    name: String,
    tag: String,
    parent: Parent,
    tracks: Vec<usize>,
    menu: Menu,
}

impl TagMenu {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        tag: &str,
        parent: Option<String>
    ) -> Components {
        Components::TagMenu(TagMenu::new(name, color, focus_color, tag, parent))
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        tag: &str,
        parent: Option<String>
    ) -> TagMenu {
        TagMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            tag: tag.to_string(),
            tracks: Vec::new(),
            menu: Menu {
                color,
                focus_color,
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    pub fn spawn_update_event(&self, library: &Vec<Song>) -> Event {
        let event_tracks = self.selection(library);

        Event::ToGlobal(GlobalEvent::TagMenuUpdated(self.name.clone(), event_tracks))
    }

    pub fn set_menu_items(&mut self, library: &Vec<Song>) {
        self.menu.selection = 0;
        self.menu.items = vec!["<All>".to_string()];

        let mut items: Vec<String> = self.tracks.clone().iter()
            .filter(|id| library.get(**id) != None)
            .map(|id| match library.get(*id).unwrap().tags.get(&self.tag) {
                Some(val) => val.to_string(),
                None => "<Empty>".to_string(),
            }).collect();

        items.sort();
        items.dedup();

        self.menu.items.append(&mut items);
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
                                Some(t) => t == tag,
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
                        Some(tag) => tag == sel_tag,
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
                }
            },
            GlobalEvent::TagMenuUpdated(origin, tracks) if self.parent.is(origin) => {
                self.tracks = tracks.clone();

                self.set_menu_items(&state.library);
                tx.send(self.spawn_update_event(&state.library)).unwrap();
            },
            GlobalEvent::Database(tracks) if self.parent.is_none() => {
                self.tracks = (0..tracks.len()).collect();

                self.set_menu_items(&tracks);

                tx.send(self.spawn_update_event(&tracks)).unwrap()
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
