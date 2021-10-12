use std::sync::mpsc;
use crate::event::*;
use crate::styles::StyleTree;
use crate::playlist::Playlist;
use crate::components::{Component, menu::Menu};

pub struct PlaylistMenu {
    name: String,
    menu: Menu,
    playlists: Vec<Playlist>,
}

impl PlaylistMenu {
    pub fn new(name: &str) -> PlaylistMenu {
        PlaylistMenu {
            name: name.to_string(),
            playlists: Vec::new(),
            menu: Menu {
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    fn update_menu_items(&mut self) {
        self.menu.items = self.playlists.iter()
            .map(|pl| pl.name.clone()).collect();
    }

    fn spawn_update_event(&self) -> Event {
        Event::ToGlobal(GlobalEvent::PlaylistMenuUpdated(
            self.name.clone(),
            match self.playlists.get(self.menu.selection) {
                Some(pl) => Some(pl.clone()),
                None => None,
            },
        ))
    }
}

impl Component for PlaylistMenu {
    fn name(&self) -> &str { &self.name }

    fn handle_focus(
        &mut self,
        style_tree: &Option<StyleTree>,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            FocusEvent::Select => {
                let playlists = self.playlists.get(self.menu.selection).unwrap()
                    .tracks.clone();

                let event = Event::ToMpd(MpdEvent::AddToQueue(playlists));

                tx.send(event).unwrap()
            },
            e => {
                self.menu.handle_focus(style_tree, e, tx.clone());
                tx.send(self.spawn_update_event()).unwrap()
            },
        }

    }

    fn handle_global(
        &mut self,
        _style_tree: &Option<StyleTree>,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::Playlist(playlists) => {
                self.playlists = playlists.clone();
                self.update_menu_items();
                tx.send(self.spawn_update_event()).unwrap();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        self.menu.draw(x, y, w, h);
    }
}
