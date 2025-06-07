#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: u32, // Duration in seconds
}

impl Song {
    pub fn new(id: String, title: String, artist: String, album: String, duration: u32) -> Self {
        Self {
            id,
            title,
            artist,
            album,
            duration,
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
    pub fn get_album(&self) -> &String {
        &self.album
    }
    pub fn get_duration(&self) -> u32 {
        self.duration
    }
}
