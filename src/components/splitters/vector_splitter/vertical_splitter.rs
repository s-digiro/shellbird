use super::*;
use termion::cursor;

pub struct VerticalSplitter {
    splitter: VectorSplitter,
}

impl VerticalSplitter {
    pub fn new(name: &str, draw_borders: bool) -> VerticalSplitter {
        VerticalSplitter {
            splitter: VectorSplitter {
                draw_borders,
                name: name.to_string(),
                sel: None,
                panels: Vec::new()
            }
        }
    }
}

impl Splitter for VerticalSplitter {
    fn add(&mut self, component: Box<dyn Component>, size: Size) {
        self.splitter.add(component, size);
    }

    fn focus(&self) -> Option<&Box<dyn Component>> {
        self.splitter.focus()
    }

    fn focus_mut(&mut self) -> Option<&mut Box<dyn Component>> {
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
    fn as_splitter(&self) -> Option<&dyn Splitter> {
        Some(self)
    }

    fn as_splitter_mut(&mut self) -> Option<&mut dyn Splitter> {
        Some(self)
    }

    fn name(&self) -> &str { self.splitter.name() }

    fn handle_global(&mut self, e: &GlobalEvent, tx: mpsc::Sender<Event>) {
        self.splitter.handle_global(e, tx);
    }

    fn handle_focus(&mut self, e: &FocusEvent, tx: mpsc::Sender<Event>) {
        self.splitter.handle_focus(e, tx)
    }

    fn draw(&self,x: u16, y: u16, w: u16, h: u16) {
        let mut y = y;

        let last = self.splitter.panels.len() - 1;

        for (i, panel) in self.splitter.panels.iter().enumerate() {
            let h = match panel.size {
                Size::Percent(p) => (h * p as u16) / 100,
                Size::Absolute(h) => h,
            };
            panel.component.draw(x, y, w, h);

            y = y + h;

            if self.splitter.draw_borders && i != last {
                for j in x..(x + w) {
                    print!("{}─", cursor::Goto(j, y));
                }
                y = y + 1
            };
        }
    }
}
