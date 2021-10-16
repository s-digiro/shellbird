use mpd::Song;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Song>,
}
