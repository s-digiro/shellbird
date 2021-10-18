use super::*;
use std::sync::mpsc;
use components::*;
use event::*;

pub struct Screen {
    base: Components,
    name: String,
}

impl Screen {
    pub fn new(name: &str, base: Components) -> Screen {
        Screen {
            base,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn draw(&self) {
        let (w, h) = termion::terminal_size().unwrap();
        // h - 1 so we have room for statusline
        self.base.draw(1, 1, w, h - 1, true);
    }

    pub fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.base.handle_global(state, e, tx)
    }

    pub fn handle_screen(&mut self, e: &ScreenEvent, _tx: mpsc::Sender<Event>) {
        match e {
            ScreenEvent::FocusNext => {
                if let Components::Splitter(s) = &mut self.base {
                    s.next();
                }
            },
            ScreenEvent::FocusPrev => {
                if let Components::Splitter(s) = &mut self.base {
                    s.prev();
                }
            },
        }
    }

    pub fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.base.handle_focus(state, e, tx)
    }
}
