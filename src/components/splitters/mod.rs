use crate::components::*;
use crate::GlobalState;

mod vector_splitter;

pub use vector_splitter::HorizontalSplitter;
pub use vector_splitter::VerticalSplitter;

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
    component: Components,
}

impl Panel {
    pub fn new(size: Size, component: Components) -> Panel {
        Panel { size, component }
    }
}

pub trait Splitter: Component {
    fn focus(&self) -> Option<&Components>;
    fn focus_mut(&mut self) -> Option<&mut Components>;

    fn next(&mut self) -> MoveFocusResult;
    fn prev(&mut self) -> MoveFocusResult;
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

    fn draw(&self, x: u16, y: u16, w: u16, h: u16) {
        match self {
            Splitters::VerticalSplitter(c) => c.draw(x, y, w, h),
            Splitters::HorizontalSplitter(c) => c.draw(x, y, w, h),
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
    fn focus(&self) -> Option<&Components> {
        match self {
            Splitters::VerticalSplitter(c) => c.focus(),
            Splitters::HorizontalSplitter(c) => c.focus(),
        }
    }

    fn focus_mut(&mut self) -> Option<&mut Components> {
        match self {
            Splitters::VerticalSplitter(c) => c.focus_mut(),
            Splitters::HorizontalSplitter(c) => c.focus_mut(),
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
}
