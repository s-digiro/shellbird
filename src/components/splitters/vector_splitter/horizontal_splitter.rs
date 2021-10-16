use super::*;
use super::super::Splitters;
use crate::components::Components;
use termion::cursor;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HorizontalSplitter {
    splitter: VectorSplitter,
}

impl HorizontalSplitter {
    pub fn enumed(
        name: &str,
        draw_borders: bool,
        panels: Vec<Panel>,
    ) -> Components {
        Components::Splitter(
            Splitters::HorizontalSplitter(
                HorizontalSplitter::new(name, draw_borders, panels)
            )
        )
    }

    pub fn new(
        name: &str,
        draw_borders: bool,
        panels: Vec<Panel>
    ) -> HorizontalSplitter {
        HorizontalSplitter {
            splitter: VectorSplitter {
                draw_borders,
                name: name.to_string(),
                sel: 0,
                panels,
            }
        }
    }
}

impl Splitter for HorizontalSplitter {
    fn focus(&self) -> Option<&Components> {
        self.splitter.focus()
    }

    fn focus_mut(&mut self) -> Option<&mut Components> {
        self.splitter.focus_mut()
    }

    fn next(&mut self) -> MoveFocusResult {
        self.splitter.next()
    }

    fn prev(&mut self) -> MoveFocusResult {
        self.splitter.prev()
    }
}

impl Component for HorizontalSplitter {
    fn name(&self) -> &str { self.splitter.name() }

    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.splitter.handle_global(state, e, tx);
    }

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.splitter.handle_focus(state, e, tx)
    }

    fn draw(&self,x: u16, y: u16, w: u16, h: u16) {
        let mut x = x;
        let last = self.splitter.panels.len() as isize - 1;
        for (i, panel) in self.splitter.panels.iter().enumerate() {
            let w = match panel.size {
                Size::Percent(p) => (w * p as u16) / 100,
                Size::Absolute(w) => w,
            };
            panel.component.draw(x, y, w, h);

            x = x + w;

            if self.splitter.draw_borders && i as isize != last {
                for j in y..(y + h) {
                    print!("{}│", cursor::Goto(x, j));
                }
                x = x + 1
            };
        }
    }
}
