use std::{fs, thread, sync::mpsc};
use std::io::{self, BufReader, BufRead};

use crate::event::*;

pub fn load_style_tree_async(path: &str, tx: mpsc::Sender<Event>) {
    let path = path.to_string();

    thread::spawn(move || {
        let tree = match load_tree_from_file(&path) {
            Ok(tree) => Some(tree),
            _ => None,
        };
        tx.send(Event::ToApp(AppEvent::StyleTreeLoaded(tree))).unwrap();
    });
}

#[derive(Clone)]
#[derive(Debug)]
struct Style {
    name: String,
    depth: usize,
    children: Vec<usize>,
}

impl Style {
    fn new(name: &str, depth: usize) -> Style {
        Style {
            name: name.to_string(),
            depth,
            children: Vec::new(),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct StyleTree {
    styles: Vec<Style>,
}

impl StyleTree {
    fn new() -> StyleTree {
        StyleTree {
            styles: vec![
                Style {
                    name: "Base".to_string(),
                    depth: 0,
                    children: Vec::new(),
                }
            ],
        }
    }

    pub fn name(&self, id: usize) -> &str {
        &self.styles[id].name
    }

    pub fn depth(&self, id: usize) -> usize {
        self.styles[id].depth
    }

    pub fn children(&self, id: usize) -> Vec<usize> {
        self.styles[id].children.clone()
    }

    pub fn leaf_names(&self, id: usize) -> Vec<&str> {
        let children = self.children(id);

        if children.is_empty() {
            vec![self.name(id)]
        } else {
            let mut ret = Vec::new();

            for child in self.children(id) {
                ret.append(&mut self.leaf_names(child));
            }

            ret
        }
    }
}

pub fn load_tree_from_file(path: &str) -> Result<StyleTree, io::Error> {
    let mut tree = StyleTree::new();

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut stack: Vec<usize> = Vec::new();

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

        let parent_id = match stack.last() {
            Some(parent) => *parent,
            None => 0,
        };

        let parent_depth = tree.depth(parent_id);

        let new_id = tree.styles.len();
        let new_style = Style::new(name, parent_depth + 1);

        stack.push(new_id);
        tree.styles[parent_id].children.push(new_id);

        tree.styles.push(new_style);
    }

    Ok(tree)
}
