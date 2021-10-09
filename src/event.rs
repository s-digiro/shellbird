use mpd::Song;
use termion::event::Key;

use crate::playlist::Playlist;
use crate::styles::Style;
use crate::mode::Mode;

#[derive(Debug)]
#[derive(Clone)]
pub enum Event {
    Dummy,

    // Input Events
    Input(Key),
    Resize,

    // Menu Events
    PlaylistMenuUpdated(String, Option<Playlist>),
    TagMenuUpdated(String, Vec<Song>),
    UpdateRootStyleMenu(Vec<Style>),
    StyleMenuUpdated(String, Vec<Style>),

    // Mpd Events
    NowPlaying(Option<Song>),
    Queue(Vec<Song>),
    Playlist(Vec<Playlist>),
    Database(Vec<Song>),

    // Style Events
    StyleTreeLoaded(Option<Style>),

    // Command Events
    InvalidCommand(String),

    // Requests
    SwitchScreen(usize),
    Quit,
    Mode(Mode),

    ScreenRequest(ScreenRequest),
    MpdRequest(MpdRequest),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum MpdRequest {
    TogglePause,
    ClearQueue,
    AddToQueue(Vec<Song>),
    AddStyleToQueue(Vec<String>),
    PlayAt(Song),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ScreenRequest {
    FocusNext,
    FocusPrev,
    ComponentRequest(ComponentRequest),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ComponentRequest {
    Next,
    Prev,
    Select,
    GoTo(usize),
    GoToTop,
    GoToBottom,
    Search(String),
}
