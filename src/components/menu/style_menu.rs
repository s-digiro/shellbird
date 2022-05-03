/* TUI Component for Menus which list a set of genres of equal depth in tree
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

use crate::color::Color;
use crate::components::{
    menu::{Menu, Parent},
    Component, Components,
};
use crate::event::*;
use crate::styles::StyleTree;
use crate::GlobalState;

#[derive(Debug, PartialEq)]
pub struct StyleMenu {
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
        title: Option<String>,
        title_alignment: Alignment,
        menu_alignment: Alignment,
        parent: Option<String>,
    ) -> Components {
        Components::StyleMenu(StyleMenu::new(
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
    ) -> StyleMenu {
        StyleMenu {
            parent: Parent::new(parent),
            styles: Vec::new(),
            color,
            menu: Menu {
                name: name.to_string(),
                title,
                title_alignment,
                menu_alignment,
                color,
                focus_color,
                selection: 0,
                items: vec!["<All>".to_string()],
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
                &mut tree
                    .leaf_names(style)
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
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
            &mut self
                .styles
                .iter()
                .map(|s| style_tree.name(*s).to_string())
                .collect(),
        );
    }

    fn spawn_update_event(&self) -> Event {
        Event::ToAllComponents(ComponentEvent::StyleMenuUpdated(
            self.name().to_string(),
            self.selection(),
        ))
    }
}

impl Component for StyleMenu {
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
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Prev => {
                self.menu.prev();
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToTop => {
                self.menu.to_top();
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoToBottom => {
                self.menu.to_bottom();
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::GoTo(i) => {
                self.menu.to(*i);
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Search(s) => {
                self.menu.search(s);
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::SearchPrev(s) => {
                self.menu.search_prev(s);
                tx.send(self.spawn_update_event()).unwrap();
                tx.send(self.spawn_needs_draw_event()).unwrap();
            },
            ComponentEvent::Select => {
                if let Some(tree) = &state.style_tree {
                    tx.send(Event::ToMpd(MpdEvent::AddStyleToQueue(
                        self.selection_leaf_names(tree),
                    )))
                    .unwrap();
                }
            },
            ComponentEvent::UpdateRootStyleMenu if self.parent.is_none() => {
                if let Some(tree) = &state.style_tree {
                    self.set_items(tree, &vec![0]);
                    tx.send(self.spawn_update_event()).unwrap();
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
            },
            ComponentEvent::StyleMenuUpdated(menu, styles)
                if self.parent.is(menu) =>
            {
                if let Some(tree) = &state.style_tree {
                    self.set_items(tree, styles);
                    tx.send(self.spawn_update_event()).unwrap();
                    tx.send(self.spawn_needs_draw_event()).unwrap();
                }
            },
            ComponentEvent::Database(_tracks) => {
                tx.send(self.spawn_update_event()).unwrap();
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
