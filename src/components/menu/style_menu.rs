use std::sync::mpsc;
use crate::event::*;
use crate::components::{Component, Components, menu::{Parent, Menu}};
use crate::color::Color;
use crate::styles::StyleTree;
use crate::GlobalState;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct StyleMenu {
    name: String,
    parent: Parent,
    menu: Menu,
    styles: Vec<usize>,
    color: Color,
}

impl StyleMenu {
    pub fn enumed(
        name: &str,
        color: Color,
        focus_color: Color,
        parent: Option<String>
    ) -> Components {
        Components::StyleMenu(StyleMenu::new(name, color, focus_color, parent))
    }

    pub fn new(
        name: &str,
        color: Color,
        focus_color: Color,
        parent: Option<String>
    ) -> StyleMenu {
        StyleMenu {
            name: name.to_string(),
            parent: Parent::new(parent),
            styles: Vec::new(),
            color,
            menu: Menu {
                color,
                focus_color,
                selection: 0,
                items: vec![ "<All>".to_string()],
            },
        }
    }

    fn selection(&self) -> Vec<usize> {
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

    fn selection_leaf_names(&self, tree: &StyleTree) -> Vec<String> {
        let mut ret = Vec::new();

        for style in self.selection() {
            ret.append(
                &mut tree.leaf_names(style).iter()
                    .map(|s| s.to_string())
                    .collect()
            );
        }

        ret
    }

    fn set_items(&mut self, style_tree: &StyleTree, styles: &Vec<usize>) {
        self.styles = Vec::new();

        for style in styles {
            for genre in style_tree.children(*style) {
                self.styles.push(genre);
            }
        }

        self.update_menu_items(style_tree);
    }

    fn update_menu_items(&mut self, style_tree: &StyleTree) {
        self.menu.selection = 0;
        self.menu.items = vec!["<All>".to_string()];
        self.menu.items.append(
            &mut self.styles.iter()
                .map(|s| style_tree.name(*s).to_string())
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

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            FocusEvent::Select => {
                if let Some(tree) = &state.style_tree {
                    tx.send(
                        Event::ToMpd(MpdEvent::AddStyleToQueue(
                            self.selection_leaf_names(tree)
                        ))
                    ).unwrap();
                }
            }
            e => {
                self.menu.handle_focus(state, e, tx.clone());
                tx.send(self.spawn_update_event()).unwrap()
            }
        }
    }

    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent, tx: mpsc::Sender<Event>
    ) {
        match e {
            GlobalEvent::UpdateRootStyleMenu if self.parent.is_none() => {
                if let Some(tree) = &state.style_tree {
                    self.set_items(tree, &vec![0]);
                    tx.send(self.spawn_update_event()).unwrap();
                }
            },
            GlobalEvent::StyleMenuUpdated(menu, styles) if self.parent.is(menu) => {
                if let Some(tree) = &state.style_tree {
                    self.set_items(tree, styles);
                    tx.send(self.spawn_update_event()).unwrap();
                }
            },
            GlobalEvent::Database(_tracks) => {
                tx.send(self.spawn_update_event()).unwrap();
            },
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        self.menu.draw(x, y, w, h, focus);
    }
}
