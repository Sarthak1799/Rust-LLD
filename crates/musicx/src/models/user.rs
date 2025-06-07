#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub playlists: Vec<Playlist>, // List of playlist IDs
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub songs: Vec<String>, // List of song IDs
}

impl Playlist {
    pub fn new(id: String, name: String, songs: Vec<String>) -> Self {
        Self { id, name, songs }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_songs(&self) -> &Vec<String> {
        &self.songs
    }
}

impl User {
    pub fn new(id: String, name: String, email: String, password_hash: String) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
            playlists: Vec::new(), // Initialize with an empty vector
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }

    pub fn get_playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    pub fn add_song_to_playlist(
        &mut self,
        playlist_id: &String,
        song_id: String,
    ) -> Result<(), String> {
        for playlist in &mut self.playlists {
            if &playlist.id == playlist_id {
                playlist.songs.push(song_id);
                return Ok(());
            }
        }
        Err(format!("Playlist with ID {} not found", playlist_id))
    }
    pub fn remove_song_from_playlist(
        &mut self,
        playlist_id: &String,
        song_id: &String,
    ) -> Result<(), String> {
        for playlist in &mut self.playlists {
            if &playlist.id == playlist_id {
                playlist.songs.retain(|s| s != song_id);
                return Ok(());
            }
        }
        Err(format!("Playlist with ID {} not found", playlist_id))
    }
    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.playlists.push(playlist);
    }
    pub fn remove_playlist(&mut self, playlist_id: &String) -> Result<(), String> {
        self.playlists
            .retain(|playlist| &playlist.id != playlist_id);
        Ok(())
    }
    pub fn get_playlist(&self, playlist_id: &String) -> Option<&Playlist> {
        for playlist in &self.playlists {
            if &playlist.id == playlist_id {
                return Some(playlist);
            }
        }
        None
    }
}
