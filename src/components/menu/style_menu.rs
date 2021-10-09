use std::sync::mpsc;
use crate::event::{Event, ComponentRequest, MpdRequest};
use crate::components::{Component, menu::{Parent, Menu}};
use crate::styles::{Style};

pub struct StyleMenu {
    name: String,
    parent: Parent,
    menu: Menu,
    styles: Vec<Style>,
}

impl StyleMenu {
    pub fn new(name: &str, parent: Option<String>) -> StyleMenu {
        StyleMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            styles: Vec::new(),
            menu: Menu {
                selection: 0,
                items: Vec::new(),
            },
        }
    }

    fn set_items(&mut self, styles: &Vec<Style>) {
        self.styles = styles.clone();
        self.update_menu_items();
    }

    fn update_menu_items(&mut self) {
        self.menu.items = self.styles.iter()
            .map(|s| s.name().to_string())
            .collect();
    }

    fn spawn_update_event(&self) -> Event {
        Event::StyleMenuUpdated(
            self.name.clone(),
            match self.styles.get(self.menu.selection) {
                Some(style) => style.children(),
                None => Vec::new(),
            },
        )
    }
}

impl Component for StyleMenu {
    fn name(&self) -> &str {
        &self.name
    }

    fn handle_request(&mut self, request: &ComponentRequest, tx: mpsc::Sender<Event>) {
        match request {
            ComponentRequest::Select => match self.styles.get(self.menu.selection) {
                Some(style) => tx.send(Event::MpdRequest(
                    MpdRequest::AddStyleToQueue(style.leaves())
                )).unwrap(),
                None => (),
            }
            request => {
                self.menu.handle_request(request, tx.clone());
                tx.send(self.spawn_update_event()).unwrap()
            }
        }
    }

    fn update(&mut self, event: &Event, tx: mpsc::Sender<Event>) {
        match event {
            Event::UpdateRootStyleMenu(styles) if self.parent.is_none() => {
                self.set_items(styles);
                tx.send(self.spawn_update_event()).unwrap();
            },
            Event::StyleMenuUpdated(menu, styles) if self.parent.is(menu) => {
                self.set_items(styles);
                tx.send(self.spawn_update_event()).unwrap();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        self.menu.draw(x, y, w, h);
    }
}
