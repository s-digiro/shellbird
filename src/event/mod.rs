mod nestable_event;

pub use nestable_event::NestableEvent;

use mpd::Song;
use termion::event::Key;
use std::fmt;

use crate::playlist::Playlist;
use crate::styles::StyleTree;
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

    BindKey(String, NestableEvent),

    ToApp(AppEvent),
    ToCommandLine(CommandLineEvent),
    ToScreen(ScreenEvent),
    ToGlobal(GlobalEvent),
    ToFocus(FocusEvent),
    ToMpd(MpdEvent),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AppEvent {
    Resize,
    StyleTreeLoaded(Option<StyleTree>),
    SwitchScreen(String),
    Database(Vec<Song>),
    Quit,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CommandLineEvent {
    Echo(String),
    Mode(Mode),
    Input(Key),
    SbrcError(usize, String),
    SbrcNotFound,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ScreenEvent {
    FocusNext,
    FocusPrev,
}

#[derive(Clone)]
pub enum GlobalEvent {
    NowPlaying(Option<Song>),
    Queue(Vec<Song>),
    Playlist(Vec<Playlist>),
    Database(Vec<Song>),
    PlaylistMenuUpdated(String, Option<Playlist>),
    TagMenuUpdated(String, Vec<Song>),
    StyleMenuUpdated(String, Vec<usize>),
    UpdateRootStyleMenu,
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

#[derive(Clone)]
pub enum MpdEvent {
    TogglePause,
    ClearQueue,
    AddToQueue(Vec<Song>),
    AddStyleToQueue(Vec<String>),
    PlayAt(Song),
    Random,
}

impl fmt::Debug for GlobalEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GlobalEvent::NowPlaying(i) => write!(f, "GlobalEvent::NowPlaying({:?})", i),
            GlobalEvent::Queue(s) => write!(f, "GlobalEvent::Queue({} songs)", s.len()),
            GlobalEvent::Playlist(pl) => write!(f, "GlobalEvent::Playlist({} playlists)", pl.len()),
            GlobalEvent::Database(s) => write!(f, "GlobalEvent::Database({} songs)", s.len()),
            GlobalEvent::PlaylistMenuUpdated(t, pl) => write!(f, "GlobalEvent::PlaylistMenuUpdated({}, {} songs)", t, match pl { Some(_) => "Some", None => "None", }),
            GlobalEvent::TagMenuUpdated(t, s) => write!(f, "GlobalEvent::TagMenuUpdated({}, {} songs)", t, s.len()),
            GlobalEvent::UpdateRootStyleMenu => write!(f, "GlobalEvent::UpdateRootStyleMenu"),
            GlobalEvent::StyleMenuUpdated(t, s) => write!(f, "GlobalEvent::StyleMenuUpdated({}, {})", t, s.len()),
        }
    }
}

impl fmt::Debug for MpdEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MpdEvent::TogglePause => write!(f, "MpdEvent::TogglePause"),
            MpdEvent::ClearQueue => write!(f, "MpdEvent::ClearQueue"),
            MpdEvent::AddToQueue(songs) => write!(f, "MpdEvent::AddToQueue({} songs)", songs.len()),
            MpdEvent::AddStyleToQueue(genres) => write!(f, "MpdEvent::AddStyleToQueue({} genres)", genres.len()),
            MpdEvent::PlayAt(song) => write!(f, "MpdEvent::PlayAt({:?})", song),
            MpdEvent::Random => write!(f, "MpdEvent::Random"),
        }
    }
}
