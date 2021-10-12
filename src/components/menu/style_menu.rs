use std::sync::mpsc;
use crate::event::*;
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
                items: vec![ "<All>".to_string()],
            },
        }
    }

    fn selection(&self) -> Vec<Style> {
        if self.menu.selection == 0 {
            self.styles.clone()
        } else {
            if let Some(style) = self.styles.get(self.menu.selection - 1) {
                vec![style.clone()]
            } else {
                Vec::new()
            }
        }
    }

    fn set_items(&mut self, styles: &Vec<Style>) {
        self.styles = Vec::new();

        for style in styles {
            for genre in style.children() {
                self.styles.push(genre);
            }
        }

        self.update_menu_items();
    }

    fn update_menu_items(&mut self) {
        self.menu.items = vec!["<All>".to_string()];
        self.menu.items.append(
            &mut self.styles.iter()
                .map(|s| s.name().to_string())
                .collect()
        );
    }

    fn spawn_update_event(&self) -> Event {
        Event::ToGlobal(GlobalEvent::StyleMenuUpdated(
            self.name.clone(),
            self.selection(),
        ))
    }
}

impl Component for StyleMenu {
    fn name(&self) -> &str {
        &self.name
    }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        match e {
            FocusEvent::Select => match self.styles.get(self.menu.selection) {
                Some(style) => tx.send(Event::ToMpd(
                    MpdEvent::AddStyleToQueue(style.leaves())
                )).unwrap(),
                None => (),
            }
            e => {
                self.menu.handle_focus(e, tx.clone());
                tx.send(self.spawn_update_event()).unwrap()
            }
        }
    }

    fn handle_global(&mut self, e: &GlobalEvent, tx: mpsc::Sender<Event>) {
        match e {
            GlobalEvent::UpdateRootStyleMenu(base_style) if self.parent.is_none() => {
                if let Some(style) = base_style {
                    self.set_items(&vec![style.clone()]);
                    tx.send(self.spawn_update_event()).unwrap();
                }
            },
            GlobalEvent::StyleMenuUpdated(menu, styles) if self.parent.is(menu) => {
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
