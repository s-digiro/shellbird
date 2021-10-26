extern crate mpd;
extern crate unicode_width;
extern crate unicode_truncate;
extern crate json;
extern crate termion;

pub mod event;
pub mod components;
pub mod music;
pub mod layout_config;
pub mod signals;
pub mod color;
pub mod playlist;
pub mod styles;
pub mod command_line;
pub mod mode;

use mpd::Song;
use crate::styles::StyleTree;

pub struct GlobalState {
    pub style_tree: Option<StyleTree>,
    pub library: Vec<Song>,
    pub screen: String,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            style_tree: None,
            library: Vec::new(),
            screen: "Default".to_string(),
        }
    }
}
