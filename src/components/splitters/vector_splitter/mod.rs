use std::sync::mpsc;
use crate::GlobalState;
use crate::components::{Component, Components, ErrorBox};
use crate::event::*;
use super::{Panel, Splitter, Size, MoveFocusResult};

mod horizontal_splitter;
mod vertical_splitter;

pub use horizontal_splitter::HorizontalSplitter;
pub use vertical_splitter::VerticalSplitter;

#[derive(Debug)]
#[derive(PartialEq)]
struct VectorSplitter {
    name: String,
    panels: Vec<Panel>,
    sel: usize,
    draw_borders: bool,
}

impl VectorSplitter {
    fn sel(&self) -> Option<&Components> {
        if let Some(panel) = self.panels.get(self.sel) {
            Some(&panel.component)
        } else {
            None
        }
    }

    fn sel_mut(&mut self) -> Option<&mut Components> {
        if let None = self.panels.get(self.sel) {
            None
        } else {
            Some(&mut self.panels[self.sel].component)
        }
    }

    fn sel_next(&mut self) -> MoveFocusResult {
        if self.sel + 1 < self.panels.len() {
            self.sel = self.sel + 1;
            MoveFocusResult::Success
        } else {
            MoveFocusResult::Fail
        }
    }

    fn sel_prev(&mut self) -> MoveFocusResult {
        if self.sel as i32 - 1 >= 0 {
            self.sel = self.sel - 1;
            MoveFocusResult::Success
        } else {
            MoveFocusResult::Fail
        }
    }
}

impl Splitter for VectorSplitter {
    fn focus(&self) -> Option<&Components> {
        if let Some(component) = self.sel() {
            if let Components::Splitter(sel_splitter) = component {
                sel_splitter.focus()
            } else {
                Some(component)
            }
        } else {
            None
        }
    }

    fn focus_mut(&mut self) -> Option<&mut Components> {
        if let Some(component) = self.sel_mut() {
            if let Components::Splitter(s) = component {
                Some(
                    s.focus_mut().unwrap()
                )
            } else {
                Some(component)
            }
        } else {
            None
        }
    }

    fn next(&mut self) -> MoveFocusResult {
        match self.sel_mut() {
            Some(component) => match component {
                Components::Splitter(splitter) => match splitter.next() {
                    MoveFocusResult::Success => MoveFocusResult::Success,
                    MoveFocusResult::Fail => self.sel_next(),
                },
                _ => self.sel_next(),
            },
            None => MoveFocusResult::Fail,
        }
    }

    fn prev(&mut self) -> MoveFocusResult {
        match self.sel_mut() {
            Some(component) => match component {
                Components::Splitter(splitter) => match splitter.prev() {
                    MoveFocusResult::Success => MoveFocusResult::Success,
                    MoveFocusResult::Fail => self.sel_prev(),
                }
                _ => self.sel_prev(),
            },
            None => MoveFocusResult::Fail,
        }
    }
}

impl Component for VectorSplitter {
    fn name(&self) -> &str { &self.name }

    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        for panel in self.panels.iter_mut() {
            panel.component.handle_global(state, e, tx.clone())
        }
    }

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        if let Some(sel) = self.sel_mut() {
            sel.handle_focus(state, e, tx);
        }
    }

    fn draw(&self,x: u16, y: u16, w: u16, h: u16) {
        ErrorBox::new().draw(x, y, w, h);
    }
}
