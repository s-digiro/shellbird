extern crate mpd;
extern crate json;
extern crate termion;

pub mod event;
pub mod components;
pub mod music;
pub mod layout_config;
pub mod screen;
pub mod signals;
pub mod color;
pub mod playlist;
pub mod styles;
pub mod command_line;
pub mod mode;

use crate::styles::StyleTree;
use mpd::Song;

pub struct GlobalState {
    pub style_tree: Option<StyleTree>,
    pub library: Vec<Song>,
}

