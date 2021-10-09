use mpd::Song;

#[derive(Clone)]
#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Song>,
}
