/* Contains functionality for styles: genre sub genre trees
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

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::{fs, sync::mpsc, thread};

use mpd::Song;

use crate::event::*;

pub fn load_style_tree_async(path: &str, tx: mpsc::Sender<Event>) {
    let path = path.to_string();

    thread::spawn(move || {
        let tree = match load_tree_from_file(&path) {
            Ok(tree) => Some(tree),
            _ => None,
        };
        tx.send(Event::ToApp(AppEvent::StyleTreeLoaded(tree)))
            .unwrap();
    });
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct StyleTree {
    styles: Vec<Style>,
    tracks: HashMap<Option<String>, Vec<Song>>,
}

impl StyleTree {
    fn new() -> StyleTree {
        StyleTree {
            styles: vec![Style {
                name: "Base".to_string(),
                depth: 0,
                children: Vec::new(),
            }],
            tracks: HashMap::new(),
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

    pub fn set_tracks(&mut self, tracks: Vec<Song>) {
        self.tracks.clear();

        for track in tracks {
            let key = match track.tags.get("Genre") {
                Some(s) => Some(s.to_string()),
                None => None,
            };

            let bucket = match self.tracks.get_mut(&key) {
                Some(vec) => vec,
                None => {
                    self.tracks.insert(key.clone(), Vec::new());
                    self.tracks.get_mut(&key).unwrap()
                },
            };

            bucket.push(track);
        }
    }

    pub fn tracks(&self, key: &Option<String>) -> Option<&Vec<Song>> {
        self.tracks.get(key)
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
