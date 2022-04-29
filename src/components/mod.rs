/* Contains functionality for TUI components and groups all
   TUI components into one module
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

use crate::event::*;
use crate::GlobalState;
use std::fmt;
use std::sync::mpsc;
use termion::cursor;

mod empty_space;
mod error_box;
mod menu;
mod place_holder;
mod splitters;
mod tag_display;
mod tag_editor;
mod title_display;

pub use empty_space::EmptySpace;
pub use error_box::ErrorBox;
pub use menu::playlist_menu::PlaylistMenu;
pub use menu::queue::Queue;
pub use menu::style_menu::StyleMenu;
pub use menu::tag_menu::TagMenu;
pub use menu::track_menu::TrackMenu;
pub use place_holder::PlaceHolder;
pub use splitters::HorizontalSplitter;
pub use splitters::MoveFocusResult;
pub use splitters::Panel;
pub use splitters::Size;
pub use splitters::Splitter;
pub use splitters::Splitters;
pub use splitters::VerticalSplitter;
pub use tag_display::TagDisplay;
pub use tag_editor::TagEditor;
pub use title_display::TitleDisplay;

pub trait Component: fmt::Debug + PartialEq {
    fn spawn_needs_draw_event(&self) -> Event {
        Event::ToScreen(ScreenEvent::NeedsRedraw(self.name().to_string()))
    }

    fn handle(&mut self, _state: &GlobalState, e: &ComponentEvent, _tx: mpsc::Sender<Event>) {
        match e {
            ComponentEvent::Draw(x, y, w, h, focus) => {
                self.draw(*x, *y, *w, *h, focus.as_str() == self.name())
            }
            _ => (),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool);

    fn border(&self, x: u16, y: u16, w: u16, h: u16) {
        print!(
            "{}{}{}{}",
            cursor::Goto(x, y),
            "┌",
            "─".to_string().repeat((w - 2).into()),
            "┐",
        );

        for line in (y + 1)..(y + h - 1) {
            print!("{}{}", cursor::Goto(x, line), "│",);
            print!("{}{}", cursor::Goto(x + w - 1, line), "│",);
        }

        print!(
            "{}{}{}{}",
            cursor::Goto(x, y + h - 1),
            "└",
            "─".repeat((w - 2).into()),
            "┘",
        );
    }

    fn clear(&self, x: u16, y: u16, w: u16, h: u16) {
        let mut buffer = String::new();

        for line in y..(y + h) {
            buffer.push_str(&format!(
                "{}{}",
                cursor::Goto(x, line),
                " ".repeat(w as usize),
            ));
        }

        print!("{}", buffer);
    }

    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub enum Components {
    PlaceHolder(PlaceHolder),
    TagEditor(TagEditor),
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
    fn handle(&mut self, state: &GlobalState, e: &ComponentEvent, tx: mpsc::Sender<Event>) {
        match self {
            Components::PlaceHolder(c) => c.handle(state, e, tx),
            Components::TagEditor(c) => c.handle(state, e, tx),
            Components::EmptySpace(c) => c.handle(state, e, tx),
            Components::ErrorBox(c) => c.handle(state, e, tx),
            Components::TitleDisplay(c) => c.handle(state, e, tx),
            Components::TagDisplay(c) => c.handle(state, e, tx),
            Components::Queue(c) => c.handle(state, e, tx),
            Components::PlaylistMenu(c) => c.handle(state, e, tx),
            Components::TrackMenu(c) => c.handle(state, e, tx),
            Components::TagMenu(c) => c.handle(state, e, tx),
            Components::StyleMenu(c) => c.handle(state, e, tx),
            Components::Splitter(x) => x.handle(state, e, tx),
        }
    }

    fn draw(&self, x: u16, y: u16, w: u16, h: u16, focus: bool) {
        match self {
            Components::PlaceHolder(c) => c.draw(x, y, w, h, focus),
            Components::TagEditor(c) => c.draw(x, y, w, h, focus),
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
            Components::TagEditor(c) => c.border(x, y, w, h),
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
            Components::TagEditor(c) => c.name(),
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
