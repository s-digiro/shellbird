use super::*;
use std::sync::mpsc;
use components::*;
use event::ScreenRequest;

pub struct Screen {
    base: Box<dyn Component>,
    name: String,
}

impl Screen {
    pub fn new(name: &str, base: Box<dyn Component>) -> Screen {
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
        self.base.draw(1, 1, w, h);
    }

    pub fn update(&mut self, event: &Event, tx: mpsc::Sender<Event>) {
        self.base.update(event, tx)
    }

    pub fn handle_request(
        &mut self,
        request: &ScreenRequest,
        tx: mpsc::Sender<Event>,
    ) {
        match request {
            ScreenRequest::FocusNext => {
                if let Some(splitter) = self.base.as_splitter_mut() {
                    splitter.next();
                }
            },
            ScreenRequest::FocusPrev => {
                if let Some(splitter) = self.base.as_splitter_mut() {
                    splitter.prev();
                }
            },
            ScreenRequest::ComponentRequest(request) => {
                self.base.handle_request(request, tx)
            },
        }
    }
}

pub fn new_now_playing_screen() -> Screen {
    let mut base = HorizontalSplitter::new("NowPlayingScreen-Base", false);

    let mut center_splitter = VerticalSplitter::new("NowPlayingScreen-CenterSplitter", false);

    center_splitter.add(
        Box::new(PlaceHolder::new("NowPlayingScreen-AlbumArt")),
        Size::Percent(30),
    );


    center_splitter.add(
        Box::new(TagDisplay::new("NowPlayingScreen-Artist", "Artist")),
        Size::Absolute(1),
    );

    center_splitter.add(
        Box::new(TitleDisplay::new("NowPlayingScreen-Title")),
        Size::Absolute(1),
    );

    center_splitter.add(
        Box::new(TagDisplay::new("NowPlayingScreen-Album", "Album")),
        Size::Absolute(1),
    );

    base.add(
        Box::new(EmptySpace::new("NowPlayingScreen-EmptySpace1")),
        Size::Percent(33),
    );

    base.add(
        Box::new(center_splitter),
        Size::Percent(33),
    );

    base.add(
        Box::new(EmptySpace::new("NowPlayingScreen-EmptySpace2")),
        Size::Percent(33),
    );

    Screen::new("NowPlayingScreen", Box::new(base))

}

pub fn new_queue_screen() -> Screen {
    Screen::new("QueueScreen", Box::new(Queue::new("QueueScreen-Queue")))
}

pub fn new_playlist_view_screen() -> Screen {
    let mut split = HorizontalSplitter::new("PlaylistViewScreen-Base", true);

    split.add(
        Box::new(PlaylistMenu::new("PlaylistViewScreen-PlaylistMenu")),
        Size::Percent(40),
    );

    split.add(
        Box::new(TrackMenu::new(
            "PlaylistViewScreen-TrackMenu",
            Some("PlaylistView-PlaylistMenu".to_string())
        )),
        Size::Percent(60),
    );

    Screen::new("PlaylistViewScreen", Box::new(split))
}

pub fn new_library_view_screen() -> Screen {
    let mut base = HorizontalSplitter::new("LibraryViewScreen-Base", true);

    base.add(
        Box::new(TagMenu::new(
            "LibraryViewScreen-ArtistMenu",
            "AlbumArtist",
            None,
        )),
        Size::Percent(33),
    );

    base.add(
        Box::new(TagMenu::new(
            "LibraryViewScreen-AlbumMenu",
            "Album",
            Some("LibraryViewScreen-ArtistMenu".to_string()),
        )),
        Size::Percent(33),
    );

    base.add(
        Box::new(TrackMenu::new(
            "LibraryViewScreen-TrackMenu",
            Some("LibraryViewScreen-AlbumMenu".to_string()),
        )),
        Size::Percent(33),
    );

    Screen::new("LibraryViewScreen", Box::new(base))
}

pub fn new_style_view_screen() -> Screen {
    let mut base = HorizontalSplitter::new("StyleViewScreen-Base", true);

    base.add(
        Box::new(StyleMenu::new(
            "StyleViewScreen-StyleMenu1",
            None,
        )),
        Size::Percent(20),
    );

    base.add(
        Box::new(StyleMenu::new(
            "StyleViewScreen-StyleMenu2",
            Some(String::from("StyleViewScreen-StyleMenu1")),
        )),
        Size::Percent(20),
    );


    base.add(
        Box::new(StyleMenu::new(
            "StyleViewScreen-StyleMenu3",
            Some(String::from("StyleViewScreen-StyleMenu2")),
        )),
        Size::Percent(20),
    );


    base.add(
        Box::new(StyleMenu::new(
            "StyleViewScreen-StyleMenu4",
            Some(String::from("StyleViewScreen-StyleMenu3")),
        )),
        Size::Percent(20),
    );


    base.add(
        Box::new(StyleMenu::new(
            "StyleViewScreen-StyleMenu5",
            Some(String::from("StyleViewScreen-StyleMenu4")),
        )),
        Size::Percent(20),
    );

    Screen::new("StyleViewScreen", Box::new(base))
}
