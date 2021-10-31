use super::*;
use super::super::Splitters;

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
    fn contains(&self, key: &str) -> bool {
        self.splitter.contains(key)
    }

    fn children(&self) -> Vec<&str> {
        self.splitter.children()
    }

    fn focus(&self) -> Option<&str> {
        self.splitter.focus()
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
                    let inner_h = match panel.size {
                        Size::Percent(p) => (inner_h * p as u16) / 100,
                        Size::Absolute(inner_h) => inner_h,
                        Size::Remainder => (inner_h - inner_y) + 1,
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


                    inner_y = inner_y + inner_h;

                    if self.splitter.draw_borders && i != last {
                        inner_y = inner_y + 1;
                    }
                }
            },
            e => self.splitter.handle(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, _focus: bool) {
        let mut inner_y = y;
        let mut inner_h = h;

        if self.splitter.draw_borders {
            self.border(x, y, w, h);
            inner_y = inner_y + 1;
            inner_h = inner_h - 2;
        }

        let last = self.splitter.panels.len() - 1;

        for (i, panel) in self.splitter.panels.iter().enumerate() {
            let inner_h = match panel.size {
                Size::Percent(p) => (inner_h * p as u16) / 100,
                Size::Absolute(inner_h) => inner_h,
                Size::Remainder => match self.splitter.draw_borders {
                    false => inner_h - inner_y,
                    true => (inner_h - inner_y) + 1,
                },
            };

            inner_y = inner_y + inner_h;

            if self.splitter.draw_borders && i != last {
                super::draw_horizontal_line(x, inner_y, w);
                inner_y = inner_y + 1;
            }
        }

        if self.splitter.draw_borders {
            super::draw_bottom_border(x, y + h - 1, w);
        }
    }
}
