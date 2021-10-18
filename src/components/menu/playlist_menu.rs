use std::sync::mpsc;
use crate::event::*;
use crate::GlobalState;
use crate::playlist::Playlist;
use crate::components::{Component, Components, menu::Menu};
use crate::color::Color;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PlaylistMenu {
    name: String,
    menu: Menu,
    playlists: Vec<Playlist>,
}

impl PlaylistMenu {
    pub fn enumed(name: &str, color: Color, focus_color: Color) -> Components {
        Components::PlaylistMenu(PlaylistMenu::new(name, color, focus_color))
    }

    pub fn new(name: &str, color: Color, focus_color: Color) -> PlaylistMenu {
        PlaylistMenu {
            name: name.to_string(),
            playlists: Vec::new(),
            menu: Menu {
                color,
                focus_color,
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
        state: &GlobalState,
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
                self.menu.handle_focus(state, e, tx.clone());
                tx.send(self.spawn_update_event()).unwrap()
            },
        }

    }

    fn handle_global(
        &mut self,
        _state: &GlobalState,
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

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
