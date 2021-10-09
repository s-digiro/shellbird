use std::sync::mpsc;
use crate::components::{Component, ErrorBox};
use crate::event::*;
use super::{Panel, Splitter, Size, MoveFocusResult};

mod horizontal_splitter;
mod vertical_splitter;

pub use horizontal_splitter::HorizontalSplitter;
pub use vertical_splitter::VerticalSplitter;

struct VectorSplitter {
    name: String,
    panels: Vec<Panel>,
    sel: Option<usize>,
    draw_borders: bool,
}

impl VectorSplitter {
    fn sel(&self) -> Option<&Box<dyn Component>> {
        if let Some(sel) = self.sel {
            if let Some(panel) = self.panels.get(sel) {
                Some(&panel.component)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn sel_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        if let Some(sel) = self.sel {
            if let None = self.panels.get(sel) {
                None
            } else {
                Some(&mut self.panels[sel].component)
            }
        } else {
            None
        }
    }

    fn sel_next(&mut self) -> MoveFocusResult {
        if let Some(sel) = self.sel {
            if sel + 1 < self.panels.len() {
                self.sel = Some(sel + 1);
                MoveFocusResult::Success
            } else { MoveFocusResult::Fail
            } } else { MoveFocusResult::Fail
        }
    }

    fn sel_prev(&mut self) -> MoveFocusResult {
        if let Some(sel) = self.sel {
            if sel as i32 - 1 >= 0 {
                self.sel = Some(sel - 1);
                MoveFocusResult::Success
            } else {
                MoveFocusResult::Fail
            }
        } else {
            MoveFocusResult::Fail
        }
    }
}

impl Splitter for VectorSplitter {
    fn add(&mut self, component: Box<dyn Component>, size: Size) {
        self.panels.push(Panel { size, component, });

        if let None = self.sel {
            self.sel = Some(0);
        }
    }

    fn focus(&self) -> Option<&Box<dyn Component>> {
        if let Some(component) = self.sel() {
            if let Some(sel_splitter) = component.as_splitter() {
                sel_splitter.focus()
            } else {
                Some(component)
            }
        } else {
            None
        }
    }

    fn focus_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        if let Some(component) = self.sel_mut() {
            if let None = component.as_splitter_mut() {
                Some(component)
            } else {
                Some(
                    component.as_splitter_mut().unwrap().focus_mut().unwrap()
                )
            }
        } else {
            None
        }
    }

    fn next(&mut self) -> MoveFocusResult {
        match self.sel_mut() {
            Some(component) => match component.as_splitter_mut() {
                Some(splitter) => match splitter.next() {
                    MoveFocusResult::Success => MoveFocusResult::Success,
                    MoveFocusResult::Fail => self.sel_next(),
                }
                None => self.sel_next(),
            },
            None => MoveFocusResult::Fail,
        }
    }

    fn prev(&mut self) -> MoveFocusResult {
        match self.sel_mut() {
            Some(component) => match component.as_splitter_mut() {
                Some(splitter) => match splitter.prev() {
                    MoveFocusResult::Success => MoveFocusResult::Success,
                    MoveFocusResult::Fail => self.sel_prev(),
                }
                None => self.sel_prev(),
            },
            None => MoveFocusResult::Fail,
        }
    }
}

impl Component for VectorSplitter {
    fn as_splitter(&self) -> Option<&dyn Splitter> {
        Some(self)
    }

    fn as_splitter_mut(&mut self) -> Option<&mut dyn Splitter> {
        Some(self)
    }

    fn name(&self) -> &str { &self.name }

    fn handle_global(&mut self, e: &GlobalEvent, tx: mpsc::Sender<Event>) {
        for panel in self.panels.iter_mut() {
            panel.component.handle_global(e, tx.clone())
        }
    }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        if let Some(sel) = self.sel_mut() {
            sel.handle_focus(e, tx);
        }
    }

    fn draw(&self,x: u16, y: u16, w: u16, h: u16) {
        ErrorBox::new().draw(x, y, w, h);
    }
}
