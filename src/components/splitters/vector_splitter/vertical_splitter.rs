use super::*;
use super::super::Splitters;
use termion::cursor;

use crate::GlobalState;
use crate::components::Components;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct VerticalSplitter {
    splitter: VectorSplitter,
}

impl VerticalSplitter {
    pub fn enumed(
        name: &str,
        draw_borders: bool,
        panels: Vec<Panel>,
    ) -> Components {
        Components::Splitter(
            Splitters::VerticalSplitter(
                VerticalSplitter::new(name, draw_borders, panels)
            )
        )
    }

    pub fn new(
        name: &str,
        draw_borders: bool,
        panels: Vec<Panel>,
    ) -> VerticalSplitter {
        VerticalSplitter {
            splitter: VectorSplitter {
                draw_borders,
                name: name.to_string(),
                sel: 0,
                panels,
            }
        }
    }
}

impl Splitter for VerticalSplitter {
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

impl Component for VerticalSplitter {
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
        let mut y = y;

        let last = self.splitter.panels.len() as isize - 1;

        for (i, panel) in self.splitter.panels.iter().enumerate() {
            let h = match panel.size {
                Size::Percent(p) => (h * p as u16) / 100,
                Size::Absolute(h) => h,
            };
            panel.component.draw(x, y, w, h);

            y = y + h;

            if self.splitter.draw_borders && i as isize != last {
                for j in x..(x + w) {
                    print!("{}─", cursor::Goto(j, y));
                }
                y = y + 1
            };
        }
    }
}
