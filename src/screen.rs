/* Represents a screen in the application
   Copyright (C) 2020-2021 Sean DiGirolamo

This file is part of Shellbird.

Shellbird is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation; either version 3, or (at your option) any
later version.

Shellbird is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Shellbird; see the file COPYING.  If not see
<http://www.gnu.org/licenses/>.  */

use std::collections::{VecDeque, HashMap};
use std::fmt;
use crate::components::{Splitter, Components, Component, MoveFocusResult};

pub struct Screen {
    name: String,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Screen {
    pub fn new(name: &str) -> Screen {
        Screen {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn focus<'a>(
        &self,
        components: &'a HashMap<String, Components>
    ) -> String {
        let stack = construct_focus_stack(&self.name, components);

        let key = stack.back().unwrap().to_string();

        match components.get(&key) {
            Some(Components::Splitter(s)) => match s.focus() {
                Some(focus) => focus.to_string(),
                None => s.name().to_string(),
            },
            _ => key.to_string(),
        }
    }

    pub fn focus_next(&self, components: &mut HashMap<String, Components>) {
        let mut stack = construct_focus_stack(&self.name, components);

        let mut res = MoveFocusResult::Fail;
        while res == MoveFocusResult::Fail {
            if let Some(key) = stack.pop_back() {
                let c = match components.get_mut(&key) {
                    Some(Components::Splitter(s)) => s,
                    _ => break,
                };

                res = c.next();
            } else {
                break;
            }
        }
    }

    pub fn focus_prev(&self, components: &mut HashMap<String, Components>) {
        let mut stack = construct_focus_stack(&self.name, components);

        let mut res = MoveFocusResult::Fail;
        while res == MoveFocusResult::Fail {
            if let Some(key) = stack.pop_back() {
                let c = match components.get_mut(&key) {
                    Some(Components::Splitter(s)) => s,
                    _ => break,
                };

                res = c.prev();
            } else {
                break;
            }
        }
    }

    pub fn contains(
        &self,
        key: &str,
        components: &HashMap<String, Components>
    ) -> bool {
        if self.name == key {
            true
        } else {
            match components.get(&self.name) {
                Some(c) => splitter_contains(c, key, components),
                None => false,
            }
        }
    }
}

fn splitter_contains(
    component: &Components,
    key: &str,
    components: &HashMap<String, Components>,
) -> bool {
    match component {
        Components::Splitter(splitter) => {
            if splitter.name() == key {
                true
            } else {
                for child in splitter.children() {
                    if let Some(component) = components.get(child) {
                        if splitter_contains(component, key, components) {
                            return true
                        }
                    }
                }

                false
            }
        },
        _ => component.name() == key,
    }
}

fn construct_focus_stack(
    root: &str,
    components: &HashMap<String, Components>
) -> VecDeque<String> {
    let mut stack: VecDeque<String> = VecDeque::new();
    stack.push_back(root.to_string());
    loop {
        let back = stack.back().unwrap().to_string();
        if let Some(Components::Splitter(s)) = components.get(&back) {
            if let Some(focus) = s.focus() {
                if let Some(Components::Splitter(s)) = components.get(focus) {
                    stack.push_back(s.name().to_string());
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break
        }
    }

    stack
}
