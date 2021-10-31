use super::*;
use super::super::Splitters;
use crate::components::Components;

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
    fn focus(&self) -> Option<&str> {
        self.splitter.focus()
    }

    fn next(&mut self) -> MoveFocusResult {
        self.splitter.next()
    }

    fn prev(&mut self) -> MoveFocusResult {
        self.splitter.prev()
    }

    fn contains(&self, key: &str) -> bool {
        self.splitter.contains(key)
    }

    fn children(&self) -> Vec<&str> {
        self.splitter.children()
    }
}

impl Component for HorizontalSplitter {
    fn name(&self) -> &str { self.splitter.name() }

    fn handle(
        &mut self,
        state: &GlobalState,
        e: &ComponentEvent,
        tx: mpsc::Sender<Event>
    ) {
        match e {
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, false);

                let mut inner_x = *x;
                let mut inner_y = *y;
                let mut inner_w = *w;
                let mut inner_h = *h;

                if self.splitter.draw_borders {
                    inner_x = inner_x + 1;
                    inner_y = inner_y + 1;
                    inner_w = inner_w - 2;
                    inner_h = inner_h - 2;
                }

                let last = self.splitter.panels.len() - 1;

                for (i, panel) in self.splitter.panels.iter().enumerate() {
                    let inner_w = match panel.size {
                        Size::Percent(p) => (inner_w * p as u16) / 100,
                        Size::Absolute(inner_w) => inner_w,
                        Size::Remainder => match self.splitter.draw_borders {
                            false => w - inner_x + 1,
                            true => w - inner_x,
                        },
                    };

                    tx.send(
                        Event::ToComponent(
                            panel.key.to_string(),
                            ComponentEvent::Draw(
                                inner_x,
                                inner_y,
                                inner_w,
                                inner_h,
                                focus.to_string(),
                            ),
                        ),
                    ).unwrap();

                    inner_x = inner_x + inner_w;

                    if self.splitter.draw_borders && i != last {
                        inner_x = inner_x + 1;
                    }
                }
            },
            e => self.splitter.handle(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        let mut inner_x = x;
        let mut inner_w = w;

        if self.splitter.draw_borders {
            self.border(x, y, w, h);
            inner_x = inner_x + 1;
            inner_w = inner_w - 2;
        }

        let last = self.splitter.panels.len() - 1;

        for (i, panel) in self.splitter.panels.iter().enumerate() {
            let inner_w = match panel.size {
                Size::Percent(p) => (inner_w * p as u16) / 100,
                Size::Absolute(inner_w) => inner_w,
                Size::Remainder => w - inner_x + 1,
            };

            inner_x = inner_x + inner_w;

            if self.splitter.draw_borders && i != last {
                super::draw_vertical_line(inner_x, y, h);
                inner_x = inner_x + 1;
            }
        }

        if self.splitter.draw_borders {
            super::draw_right_border(x + w - 1, y, h);
        }
    }
}
