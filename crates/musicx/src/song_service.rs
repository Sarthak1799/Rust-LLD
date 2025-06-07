use std::collections::HashMap;

use crate::models::{album::Album, artist::Artist, song::Song};

#[derive(Debug)]
pub struct SongService {
    songs: HashMap<String, Song>,
    artists: HashMap<String, Artist>,
    albums: HashMap<String, Album>,
    user_song_tracks: HashMap<String, Vec<String>>, // Maps user IDs to song IDs
}

#[derive(Debug)]
pub struct SongServiceError {
    pub message: String,
}
impl SongServiceError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl SongService {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            artists: HashMap::new(),
            albums: HashMap::new(),
            user_song_tracks: HashMap::new(),
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.insert(song.get_id().clone(), song);
    }

    pub fn get_song(&self, id: &String) -> Option<&Song> {
        self.songs.get(id)
    }

    pub fn remove_song(&mut self, id: &String) {
        self.songs.remove(id);
    }

    pub fn add_artist(&mut self, artist: Artist) {
        self.artists.insert(artist.get_id().clone(), artist);
    }

    pub fn get_artist(&self, id: &String) -> Option<&Artist> {
        self.artists.get(id)
    }
    pub fn remove_artist(&mut self, id: &String) {
        self.artists.remove(id);
    }
    pub fn add_album(&mut self, album: Album) {
        self.albums.insert(album.get_id().clone(), album);
    }
    pub fn get_album(&self, id: &String) -> Option<&Album> {
        self.albums.get(id)
    }
    pub fn remove_album(&mut self, id: &String) {
        self.albums.remove(id);
    }

    pub fn get_all_songs(&self) -> Vec<Song> {
        self.songs.values().cloned().collect()
    }

    pub fn get_all_artists(&self) -> Vec<Artist> {
        self.artists.values().cloned().collect()
    }
    pub fn get_all_albums(&self) -> Vec<Album> {
        self.albums.values().cloned().collect()
    }

    pub fn add_song_to_user_tracks(
        &mut self,
        user_id: &String,
        song_id: String,
    ) -> Result<(), SongServiceError> {
        let tracks = self
            .user_song_tracks
            .entry(user_id.clone())
            .or_insert_with(Vec::new);

        tracks.push(song_id.clone());
        Ok(())
    }
    pub fn recommend_songs_for_user(
        &self,
        user_id: &String,
        recommendation_algorithm: &dyn SongRecommendation,
    ) -> Result<Vec<Song>, SongServiceError> {
        let tracks = self
            .user_song_tracks
            .get(user_id)
            .ok_or(SongServiceError::new("User not found".to_string()))?;

        let recommended_songs =
            recommendation_algorithm.recommend_songs(tracks.clone(), self.songs.clone())?;

        Ok(recommended_songs)
    }
}

pub trait SongRecommendation {
    fn recommend_songs(
        &self,
        tracks: Vec<String>,
        songs: HashMap<String, Song>,
    ) -> Result<Vec<Song>, SongServiceError>;
}

#[derive(Debug, Clone)]
pub struct SimpleRecommendation;
impl SongRecommendation for SimpleRecommendation {
    fn recommend_songs(
        &self,
        tracks: Vec<String>,
        songs: HashMap<String, Song>,
    ) -> Result<Vec<Song>, SongServiceError> {
        let mut recommended_songs = Vec::new();
        for track_id in tracks {
            if let Some(song) = songs.get(&track_id) {
                recommended_songs.push(song.clone());
            }
        }
        Ok(recommended_songs)
    }
}
