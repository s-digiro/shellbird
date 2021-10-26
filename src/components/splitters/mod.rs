use crate::components::*;
use crate::GlobalState;

mod vector_splitter;

pub use vector_splitter::HorizontalSplitter;
pub use vector_splitter::VerticalSplitter;

#[derive(PartialEq)]
pub enum MoveFocusResult {
    Success,
    Fail,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Size {
    Percent(u8),
    Absolute(u16),
    Remainder,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Panel {
    size: Size,
    key: String,
}

impl Panel {
    pub fn new(size: Size, key: String) -> Panel {
        Panel { size, key }
    }
}

pub trait Splitter: Component {
    fn focus(&self) -> Option<&str>;

    fn next(&mut self) -> MoveFocusResult;
    fn prev(&mut self) -> MoveFocusResult;

    fn contains(&self, key: &str) -> bool;
    fn children(&self) -> Vec<&str>;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Splitters {
    VerticalSplitter(VerticalSplitter),
    HorizontalSplitter(HorizontalSplitter),
}

impl Component for Splitters {
    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        match self {
            Splitters::VerticalSplitter(c) => c.handle_global(state, e, tx),
            Splitters::HorizontalSplitter(c) => c.handle_global(state, e, tx),
        }
    }

    fn handle_component(
        &mut self,
        state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>
    ) {
        match self {
            Splitters::VerticalSplitter(c) => c.handle_component(state, e, tx),
            Splitters::HorizontalSplitter(c) => c.handle_component(state, e, tx),
        }
    }

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match self {
            Splitters::VerticalSplitter(c) => c.handle_focus(state, e, tx),
            Splitters::HorizontalSplitter(c) => c.handle_focus(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        match self {
            Splitters::VerticalSplitter(c) => c.draw(x, y, w, h, focus),
            Splitters::HorizontalSplitter(c) => c.draw(x, y, w, h, focus),
        }
    }

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        match self {
            Splitters::VerticalSplitter(c) => c.border(x, y, w, h),
            Splitters::HorizontalSplitter(c) => c.border(x, y, w, h),
        }
    }

    fn name(&self) -> &str {
        match self {
            Splitters::VerticalSplitter(c) => c.name(),
            Splitters::HorizontalSplitter(c) => c.name(),
        }
    }
}


impl Splitter for Splitters {
    fn focus(&self) -> Option<&str> {
        match self {
            Splitters::VerticalSplitter(c) => c.focus(),
            Splitters::HorizontalSplitter(c) => c.focus(),
        }
    }

    fn next(&mut self) -> MoveFocusResult {
        match self {
            Splitters::VerticalSplitter(c) => c.next(),
            Splitters::HorizontalSplitter(c) => c.next(),
        }
    }

    fn prev(&mut self) -> MoveFocusResult {
        match self {
            Splitters::VerticalSplitter(c) => c.prev(),
            Splitters::HorizontalSplitter(c) => c.prev(),
        }
    }
    fn contains(&self, key: &str) -> bool {
        match self {
            Splitters::VerticalSplitter(c) => c.contains(key),
            Splitters::HorizontalSplitter(c) => c.contains(key),
        }
    }

    fn children(&self) -> Vec<&str> {
        match self {
            Splitters::VerticalSplitter(c) => c.children(),
            Splitters::HorizontalSplitter(c) => c.children(),
        }
    }
}
