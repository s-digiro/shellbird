use std::{fs, thread, sync::mpsc};
use std::io::{self, BufReader, BufRead};

use crate::event::Event;

pub fn load_style_tree_async(path: &str, tx: mpsc::Sender<Event>) {
    let path = path.to_string();

    thread::spawn(move || {
        let tree = match load_tree_from_file(&path) {
            Ok(tree) => Some(tree),
            _ => None,
        };
        tx.send(Event::StyleTreeLoaded(tree)).unwrap();
    });
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Style {
    name: String,
    depth: usize,
    children: Vec<Style>,
}

impl Style {
    pub fn new(name: &str) -> Style {
        Style {
            name: name.to_string(),
            depth: 0,
            children: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_child(&mut self, mut child: Style) {
        child.depth = self.depth + 1;
        self.children.push(child)
    }

    pub fn children(&self) -> Vec<Style> {
        self.children.clone()
    }

    pub fn name_depth(&mut self, name: &str, depth: usize) -> Option<&mut Style> {
        if self.depth == depth && self.name == name {
            return Some(self)
        }

        for child in self.children.iter_mut() {
            if let Some(child) = child.name_depth(name, depth) {
                return Some(child)
            }
        }

        None
    }

    pub fn leaves(&self) -> Vec<String> {
        if self.children.is_empty() {
            vec![self.name.clone()]
        } else {
            let mut ret = Vec::new();
            for child in self.children.iter() {
                ret.append(&mut child.leaves());
            }

            ret
        }
    }
}

pub fn load_tree_from_file(path: &str) -> Result<Style, io::Error> {
    let mut base = Style::new("root");

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut stack: Vec<(String, usize)> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut name_start = 0;
        let mut tabs = 0;

        // Count tabs and find start of word
        for (i, c) in line.chars().enumerate() {
            if c == '\t' {
                tabs += 1;
            } else {
                name_start = i;
                break;
            }
        }

        let name = &line[name_start..];

        // remove from stack until we are at parent in path
        while stack.len() > tabs {
            stack.pop();
        }

        let (parent_name, parent_depth) = match stack.last() {
            Some(parent) => parent.clone(),
            None => (base.name.clone(), base.depth),
        };

        let new_style = Style::new(name);

        stack.push((new_style.name.to_string(), parent_depth + 1));

        if let Some(parent) = base.name_depth(&parent_name, parent_depth) {
            parent.add_child(new_style);
        }
    }

    Ok(base)
}
