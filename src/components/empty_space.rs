use crate::components::{Component, Components};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct EmptySpace {
    name: String,
}

impl EmptySpace {
    pub fn enumed(name: &str) -> Components {
        Components::EmptySpace(EmptySpace::new(name))
    }

    pub fn new(name: &str) -> EmptySpace {
        EmptySpace {
            name: name.to_string(),
        }
    }

}

impl Component for EmptySpace {
    fn name(&self) -> &str { &self.name }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        self.clear(x, y, w, h);
    }
}
