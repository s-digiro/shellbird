use crate::color::Color;
use crate::components::Component;
use termion::color;

pub struct PlaceHolder {
    name: String,
    color: Color,
}

impl PlaceHolder {
    pub fn new(name: &str) -> PlaceHolder {
        PlaceHolder {
            name: name.to_string(),
            color: Color::Reset,
        }
    }

}

impl Component for PlaceHolder {
    fn name(&self) -> &str { &self.name }

    fn draw(
        &self,
        x: u16, y: u16, w: u16, h: u16,
    ) {
        println!("{}", color::Fg(self.color));
        self.border(x, y, w, h);
    }
}
