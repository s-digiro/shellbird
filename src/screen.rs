use super::*;
use std::sync::mpsc;
use components::*;
use event::*;

pub struct Screen {
    base: Components,
    name: String,
}

impl Screen {
    pub fn new(name: &str, base: Components) -> Screen {
        Screen {
            base,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn draw(&self) {
        let (w, h) = termion::terminal_size().unwrap();
        // h - 1 so we have room for statusline
        self.base.draw(1, 1, w, h - 1);
    }

    pub fn handle_global(
        &mut self,
        state: &GlobalState,
        e: &GlobalEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.base.handle_global(state, e, tx)
    }

    pub fn handle_screen(&mut self, e: &ScreenEvent, _tx: mpsc::Sender<Event>) {
        match e {
            ScreenEvent::FocusNext => {
                if let Components::Splitter(s) = &mut self.base {
                    s.next();
                }
            },
            ScreenEvent::FocusPrev => {
                if let Components::Splitter(s) = &mut self.base {
                    s.prev();
                }
            },
        }
    }

    pub fn handle_focus(
        &mut self,
        state: &GlobalState,
        e: &FocusEvent,
        tx: mpsc::Sender<Event>
    ) {
        self.base.handle_focus(state, e, tx)
    }
}

pub fn new_now_playing_screen() -> Screen {
    Screen::new(
        "NowPlayingScreen",
        HorizontalSplitter::enumed(
            "NowPlayingScreen-Base",
            false,
            vec![
                Panel::new(
                    Size::Percent(33),
                    EmptySpace::enumed("NowPlayingScreen-EmptySpace1")
                ),
                Panel::new(
                    Size::Percent(33),
                    VerticalSplitter::enumed(
                        "NowPlayingScreen-CenterSplitter",
                        false,
                        vec![
                            Panel::new(
                                Size::Percent(30),
                                PlaceHolder::enumed("NowPlayingScreen-AlbumArt"),
                            ),
                            Panel::new(
                                Size::Absolute(1),
                                TagDisplay::enumed("NowPlayingScreen-Artist", "Artist"),
                            ),
                            Panel::new(
                                Size::Absolute(1),
                                TitleDisplay::enumed("NowPlayingScreen-Title"),
                            ),
                            Panel::new(
                                Size::Absolute(1),
                                TagDisplay::enumed("NowPlayingScreen-Album", "Album"),
                            ),
                        ],
                    ),
                ),
                Panel::new(
                    Size::Percent(33),
                    EmptySpace::enumed("NowPlayingScreen-EmptySpace2"),
                ),
            ]
        ),
    )
}

pub fn new_queue_screen() -> Screen {
    Screen::new("QueueScreen", Queue::enumed("QueueScreen-Queue"))
}

pub fn new_playlist_view_screen() -> Screen {
    Screen::new(
        "PlaylistViewScreen",
        HorizontalSplitter::enumed(
            "PlaylistViewScreen-Base",
            true,
            vec![
                Panel::new(
                    Size::Percent(40),
                    PlaylistMenu::enumed("PlaylistViewScreen-PlaylistMenu"),
                ),
                Panel::new(
                    Size::Percent(60),
                    TrackMenu::enumed(
                        "PlaylistViewScreen-TrackMenu",
                        Some("PlaylistViewScreen-PlaylistMenu".to_string()),
                    )
                ),
            ],
        ),
    )
}

pub fn new_library_view_screen() -> Screen {
    Screen::new(
        "LibraryViewScreen",
        HorizontalSplitter::enumed(
            "LibraryViewScreen-Base",
            true,
            vec![
                Panel::new(
                    Size::Percent(33),
                    TagMenu::enumed(
                        "LibraryViewScreen-ArtistMenu",
                        "AlbumArtist",
                        None,
                    ),
                ),
                Panel::new(
                    Size::Percent(33),
                    TagMenu::enumed(
                        "LibraryViewScreen-AlbumMenu",
                        "Album",
                        Some("LibraryViewScreen-ArtistMenu".to_string()),
                    ),
                ),
                Panel::new(
                    Size::Percent(33),
                    TrackMenu::enumed(
                        "LibraryViewScreen-TrackMenu",
                        Some("LibraryViewScreen-TrackMenu".to_string()),
                    ),
                ),
            ],
        ),
    )
}

pub fn new_style_view_screen() -> Screen {
    Screen::new(
        "StyleViewScreen",
        VerticalSplitter::enumed(
            "StyleViewScreen-Base",
            true,
            vec![
                Panel::new(
                    Size::Percent(40),
                    HorizontalSplitter::enumed(
                        "StyleViewScreen-Filters",
                        true,
                        vec![
                            Panel::new(
                                Size::Percent(20),
                                StyleMenu::enumed(
                                    "StyleViewScreen-StyleMenu1",
                                    None,
                                ),
                            ),
                            Panel::new(
                                Size::Percent(20),
                                StyleMenu::enumed(
                                    "StyleViewScreen-StyleMenu2",
                                    Some("StyleViewScreen-StyleMenu1".to_string()),
                                ),
                            ),
                            Panel::new(
                                Size::Percent(20),
                                StyleMenu::enumed(
                                    "StyleViewScreen-StyleMenu3",
                                    Some("StyleViewScreen-StyleMenu2".to_string()),
                                ),
                            ),
                            Panel::new(
                                Size::Percent(20),
                                StyleMenu::enumed(
                                    "StyleViewScreen-StyleMenu4",
                                    Some("StyleViewScreen-StyleMenu3".to_string()),
                                ),
                            ),
                            Panel::new(
                                Size::Percent(20),
                                StyleMenu::enumed(
                                    "StyleViewScreen-StyleMenu5",
                                    Some("StyleViewScreen-StyleMenu4".to_string()),
                                ),
                            ),
                        ],
                    ),
                ),
                Panel::new(
                    Size::Percent(40),
                    TrackMenu::enumed(
                        "StyleViewScreen-Tracks",
                        Some("StyleViewScreen-StyleMenu5".to_string()),
                    ),
                ),
            ],
        ),
    )
}
