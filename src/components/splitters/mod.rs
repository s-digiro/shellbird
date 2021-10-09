use crate::components::Component;

mod vector_splitter;

pub use vector_splitter::HorizontalSplitter;
pub use vector_splitter::VerticalSplitter;

pub enum MoveFocusResult {
    Success,
    Fail,
}

pub enum Size {
    Percent(u8),
    Absolute(u16),
}

struct Panel {
    size: Size,
    component: Box<dyn Component>,
}

pub trait Splitter: Component {
    fn add(&mut self, component: Box<dyn Component>, size: Size);

    fn focus(&self) -> Option<&Box<dyn Component>>;
    fn focus_mut(&mut self) -> Option<&mut Box<dyn Component>>;

    fn next(&mut self) -> MoveFocusResult;
    fn prev(&mut self) -> MoveFocusResult;
}
