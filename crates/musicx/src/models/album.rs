#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub release_date: String,
    pub genre: String,
    pub songs: Vec<String>, // List of song IDs
}

impl Album {
    pub fn new(
        id: String,
        title: String,
        artist: String,
        release_date: String,
        genre: String,
        songs: Vec<String>,
    ) -> Self {
        Self {
            id,
            title,
            artist,
            release_date,
            genre,
            songs,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_artist(&self) -> &String {
        &self.artist
    }

    pub fn get_release_date(&self) -> &String {
        &self.release_date
    }

    pub fn get_genre(&self) -> &String {
        &self.genre
    }

    pub fn get_songs(&self) -> &Vec<String> {
        &self.songs
    }
}
