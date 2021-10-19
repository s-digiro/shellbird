use crate::event::*;
use crate::GlobalState;
use std::sync::mpsc;
use std::fmt;
use termion::cursor;

mod splitters;
mod place_holder;
mod error_box;
mod title_display;
mod tag_display;
mod menu;
mod empty_space;
mod align;

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
pub use splitters::Splitters;
pub use splitters::Size;
pub use splitters::Panel;
pub use align::Align;

pub trait Component: fmt::Debug + PartialEq {
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

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool);

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        print!("{}{}{}{}",
               cursor::Goto(x, y),
               "┌",
               "─".to_string().repeat((w - 2).into()),
               "┐",
        );

        for line in (y + 1)..(y + h - 1) {
            print!("{}{}",
               cursor::Goto(x, line),
               "│",
            );
            print!("{}{}",
                cursor::Goto(x + w - 1, line),
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
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Components {
    PlaceHolder(PlaceHolder),
    EmptySpace(EmptySpace),
    ErrorBox(ErrorBox),
    TitleDisplay(TitleDisplay),
    TagDisplay(TagDisplay),
    Queue(Queue),
    PlaylistMenu(PlaylistMenu),
    TrackMenu(TrackMenu),
    TagMenu(TagMenu),
    StyleMenu(StyleMenu),
    Splitter(Splitters),
}

impl Component for Components {
    fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        match self {
            Components::PlaceHolder(c) => c.handle_global(state, e, tx),
            Components::EmptySpace(c) => c.handle_global(state, e, tx),
            Components::ErrorBox(c) => c.handle_global(state, e, tx),
            Components::TitleDisplay(c) => c.handle_global(state, e, tx),
            Components::TagDisplay(c) => c.handle_global(state, e, tx),
            Components::Queue(c) => c.handle_global(state, e, tx),
            Components::PlaylistMenu(c) => c.handle_global(state, e, tx),
            Components::TrackMenu(c) => c.handle_global(state, e, tx),
            Components::TagMenu(c) => c.handle_global(state, e, tx),
            Components::StyleMenu(c) => c.handle_global(state, e, tx),
            Components::Splitter(x) => x.handle_global(state, e, tx),
        }
    }

    fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        match self {
            Components::PlaceHolder(c) => c.handle_focus(state, e, tx),
            Components::EmptySpace(c) => c.handle_focus(state, e, tx),
            Components::ErrorBox(c) => c.handle_focus(state, e, tx),
            Components::TitleDisplay(c) => c.handle_focus(state, e, tx),
            Components::TagDisplay(c) => c.handle_focus(state, e, tx),
            Components::Queue(c) => c.handle_focus(state, e, tx),
            Components::PlaylistMenu(c) => c.handle_focus(state, e, tx),
            Components::TrackMenu(c) => c.handle_focus(state, e, tx),
            Components::TagMenu(c) => c.handle_focus(state, e, tx),
            Components::StyleMenu(c) => c.handle_focus(state, e, tx),
            Components::Splitter(x) => x.handle_focus(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        match self {
            Components::PlaceHolder(c) => c.draw(x, y, w, h, focus),
            Components::EmptySpace(c) => c.draw(x, y, w, h, focus),
            Components::ErrorBox(c) => c.draw(x, y, w, h, focus),
            Components::TitleDisplay(c) => c.draw(x, y, w, h, focus),
            Components::TagDisplay(c) => c.draw(x, y, w, h, focus),
            Components::Queue(c) => c.draw(x, y, w, h, focus),
            Components::PlaylistMenu(c) => c.draw(x, y, w, h, focus),
            Components::TrackMenu(c) => c.draw(x, y, w, h, focus),
            Components::TagMenu(c) => c.draw(x, y, w, h, focus),
            Components::StyleMenu(c) => c.draw(x, y, w, h, focus),
            Components::Splitter(c) => c.draw(x, y, w, h, focus),
        }
    }

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        match self {
            Components::PlaceHolder(c) => c.border(x, y, w, h),
            Components::EmptySpace(c) => c.border(x, y, w, h),
            Components::ErrorBox(c) => c.border(x, y, w, h),
            Components::TitleDisplay(c) => c.border(x, y, w, h),
            Components::TagDisplay(c) => c.border(x, y, w, h),
            Components::Queue(c) => c.border(x, y, w, h),
            Components::PlaylistMenu(c) => c.border(x, y, w, h),
            Components::TrackMenu(c) => c.border(x, y, w, h),
            Components::TagMenu(c) => c.border(x, y, w, h),
            Components::StyleMenu(c) => c.border(x, y, w, h),
            Components::Splitter(c) => c.border(x, y, w, h),
        }
    }

    fn name(&self) -> &str {
        match self {
            Components::PlaceHolder(c) => c.name(),
            Components::EmptySpace(c) => c.name(),
            Components::ErrorBox(c) => c.name(),
            Components::TitleDisplay(c) => c.name(),
            Components::TagDisplay(c) => c.name(),
            Components::Queue(c) => c.name(),
            Components::PlaylistMenu(c) => c.name(),
            Components::TrackMenu(c) => c.name(),
            Components::TagMenu(c) => c.name(),
            Components::StyleMenu(c) => c.name(),
            Components::Splitter(c) => c.name(),
        }
    }
}
