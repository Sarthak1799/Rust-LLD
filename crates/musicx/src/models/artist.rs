#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genre: String,
    pub albums: Vec<String>, // List of album IDs
}
impl Artist {
    pub fn new(id: String, name: String, genre: String, albums: Vec<String>) -> Self {
        Self {
            id,
            name,
            genre,
            albums,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_genre(&self) -> &String {
        &self.genre
    }

    pub fn get_albums(&self) -> &Vec<String> {
        &self.albums
    }
}
