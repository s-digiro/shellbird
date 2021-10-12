use crate::event::*;
use crate::GlobalState;
use std::sync::mpsc;
use termion::cursor;

mod splitters;
mod place_holder;
mod error_box;
mod title_display;
mod tag_display;
mod menu;
mod empty_space;

pub use place_holder::PlaceHolder;
pub use empty_space::EmptySpace;
pub use error_box::ErrorBox;
pub use title_display::TitleDisplay;
pub use tag_display::TagDisplay;
pub use menu::queue::Queue;
pub use menu::playlist_menu::PlaylistMenu;
pub use menu::track_menu::TrackMenu;
pub use menu::tag_menu::TagMenu;
pub use menu::style_menu::StyleMenu;
pub use splitters::HorizontalSplitter;
pub use splitters::VerticalSplitter;
pub use splitters::Splitter;
pub use splitters::Size;

pub trait Component {
    fn handle_global(
        &mut self,
        _state: &GlobalState,
        _e: &GlobalEvent, _tx: mpsc::Sender<Event>
    ) { }

    fn handle_focus(
        &mut self,
        _state: &GlobalState,
        _e: &FocusEvent,
        _tx: mpsc::Sender<Event>
    ) { }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16);

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        print!("{}{}{}{}",
               cursor::Goto(x, y),
               "┌",
               "─".to_string().repeat((w - 2).into()),
               "┐",
        );

        for line in (y + 1)..(y + h - 1) {
            print!("{}{}{}{}",
               cursor::Goto(x, line),
               "│",
               " ".to_string().repeat((w - 2).into()),
               "│",
            );
        }

        print!("{}{}{}{}",
               cursor::Goto(x, y + h - 1),
               "└",
               "─".repeat((w - 2).into()),
               "┘",
        );
    }

    fn name(&self) -> &str;

    fn as_splitter(&self) -> Option<&dyn Splitter> { None }
    fn as_splitter_mut(&mut self) -> Option<&mut dyn Splitter> { None }
}
