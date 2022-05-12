/* TUI Component for Menus which list available playlists
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

use unicode_truncate::Alignment;

use mpd::Song;

use crate::color::Color;
use crate::components::{menu::Menu, Component, Components};
use crate::event::*;
use crate::playlist::Playlist;
use crate::GlobalState;

#[derive(Debug, PartialEq)]
pub struct PlaylistMenu {
    menu: Menu,
    playlists: Vec<Playlist>,
}

impl PlaylistMenu {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> Components {
        Components::PlaylistMenu(PlaylistMenu::new(
            name,
            color,
            focus_color,
            title,
            title_alignment,
            menu_alignment,
        ))
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
    ) -> PlaylistMenu {
        PlaylistMenu {
            playlists: Vec::new(),
            menu: Menu {
                title,
                name: name.to_string(),
                color,
                focus_color,
                selection: 0,
                items: Vec::new(),
                title_alignment,
                menu_alignment,
            },
        }
    }

    fn update_menu_items(&mut self) {
        self.menu.items =
            self.playlists.iter().map(|pl| pl.name.clone()).collect();
    }

    fn spawn_update_event(&self, library: &Vec<Song>) -> Event {
        let name = self.name().to_owned();
        let tracks = self
            .playlists
            .get(self.menu.selection)
            .map(|pl| pl.tracks.clone())
            .unwrap_or(Vec::new());
        let ids = library
            .iter()
            .enumerate()
            .filter(|(_, song)| tracks.contains(song))
            .map(|(i, _)| i)
            .collect();

        Event::ToAllComponents(ComponentEvent::PlaylistMenuUpdated(name, ids))
    }
}

impl Component for PlaylistMenu {
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
            ComponentEvent::OpenTags => {
                let files: Vec<&str> = self
                    .playlists
                    .get(self.menu.selection)
                    .unwrap()
                    .tracks
                    .iter()
                    .map(|s| s.file.as_str())
                    .collect();
                let ids: Vec<usize> = state
                    .library
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| files.contains(&s.file.as_str()))
                    .map(|(i, _)| i)
                    .collect();
                tx.send(Event::ToApp(AppEvent::TagUI(ids))).unwrap();
            },
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
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Select => {
                let playlists = self
                    .playlists
                    .get(self.menu.selection)
                    .unwrap()
                    .tracks
                    .clone();

                let event = Event::ToMpd(MpdEvent::AddToQueue(playlists));

                tx.send(event).unwrap()
            },
            ComponentEvent::Playlist(playlists) => {
                self.playlists = playlists.clone();
                self.update_menu_items();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::LostMpdConnection => {
                self.playlists = Vec::new();
                self.update_menu_items();
                tx.send(self.spawn_update_event(&state.library)).unwrap();
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
