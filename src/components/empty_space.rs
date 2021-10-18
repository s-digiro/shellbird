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

    fn draw(&self, _x: u16, _y: u16, _w: u16, _h: u16, _focus: bool) { }
}
