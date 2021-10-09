use mpd::Song;
use termion::event::Key;

use crate::playlist::Playlist;
use crate::styles::Style;
use crate::mode::Mode;

/* Events are sorted into different enums based on their destination
 *
 * App: Goes to and is handled by main application
 * Screen: Goes to current screen
 * Global: Goes to all components
 * Focus: Goes to focused component
 * Mpd: Goes to mpd thread to give instructions to mpd
 */

#[derive(Debug)]
#[derive(Clone)]
pub enum Event {
    Dummy,

    ToApp(AppEvent),
    ToScreen(ScreenEvent),
    ToGlobal(GlobalEvent),
    ToFocus(FocusEvent),
    ToMpd(MpdEvent),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AppEvent {
    Resize,
    StyleTreeLoaded(Option<Style>),
    InvalidCommand(String),
    CommandResponse(String),
    SwitchScreen(usize),
    Quit,
    Mode(Mode),
    Input(Key),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ScreenEvent {
    FocusNext,
    FocusPrev,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum GlobalEvent {
    NowPlaying(Option<Song>),
    Queue(Vec<Song>),
    Playlist(Vec<Playlist>),
    Database(Vec<Song>),
    PlaylistMenuUpdated(String, Option<Playlist>),
    TagMenuUpdated(String, Vec<Song>),
    UpdateRootStyleMenu(Vec<Style>),
    StyleMenuUpdated(String, Vec<Style>),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum FocusEvent {
    Next,
    Prev,
    Select,
    GoTo(usize),
    GoToTop,
    GoToBottom,
    Search(String),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum MpdEvent {
    TogglePause,
    ClearQueue,
    AddToQueue(Vec<Song>),
    AddStyleToQueue(Vec<String>),
    PlayAt(Song),
}
