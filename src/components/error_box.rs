use termion::color;
use termion::cursor;
use crate::components::{Component, Components};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ErrorBox;

impl ErrorBox {
    pub fn enumed() -> Components {
        Components::ErrorBox(ErrorBox::new())
    }

    pub fn new() -> ErrorBox {
        ErrorBox { }
    }
}

impl Component for ErrorBox {
    fn name(&self) -> &str { "Error" }

    fn draw(
        &self,
        x: u16, y: u16, w: u16, h: u16
    ) {
        let mut text = "Error".to_string();
        text.truncate((w - 2).into());

        print!("{}", color::Fg(color::Red));
        self.border(x, y, w, h);
        print!("{}{}{}",
            cursor::Goto(x + 1, y + 1),
            text,
            color::Fg(color::Reset),
        )
    }
}
