use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::models::song;
use crate::models::user::Playlist;
use crate::models::{song::Song, user::User};
use crate::song_service::{SongRecommendation, SongService};
use crate::user_service::UserService;

#[derive(Debug)]
pub struct MusicXService {
    pub user_service: UserService,
    pub song_service: SongService,
}

#[derive(Debug)]
pub struct MusicXServiceError {
    pub message: String,
}

impl MusicXService {
    pub fn new() -> Self {
        Self {
            user_service: UserService::new(),
            song_service: SongService::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.user_service.add_user(user);
    }

    pub fn get_user(&self, id: &String) -> Result<User, MusicXServiceError> {
        self.user_service
            .get_user(id)
            .ok_or(MusicXServiceError {
                message: "User not found".to_string(),
            })
            .cloned()
    }

    pub fn login_as_user(&mut self, id: &str, password: String) -> Result<(), MusicXServiceError> {
        self.user_service
            .login(id, password)
            .map_err(|e| MusicXServiceError { message: e.message })
    }
    pub fn add_song(&mut self, song: Song) {
        self.song_service.add_song(song);
    }

    pub fn remove_song(&mut self, id: &String) {
        self.song_service.remove_song(id);
    }

    pub fn add_artist(&mut self, artist: Artist) {
        self.song_service.add_artist(artist);
    }
    pub fn remove_artist(&mut self, id: &String) {
        self.song_service.remove_artist(id);
    }
    pub fn add_album(&mut self, album: Album) {
        self.song_service.add_album(album);
    }
    pub fn remove_album(&mut self, id: &String) {
        self.song_service.remove_album(id);
    }

    pub fn get_all_songs(&self) -> Vec<Song> {
        self.song_service.get_all_songs()
    }
    pub fn get_all_artists(&self) -> Vec<Artist> {
        self.song_service.get_all_artists()
    }
    pub fn get_all_albums(&self) -> Vec<Album> {
        self.song_service.get_all_albums()
    }

    pub fn add_song_for_user(
        &mut self,
        user_id: &String,
        song_id: String,
    ) -> Result<(), MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        self.song_service
            .add_song_to_user_tracks(user_id, song_id)
            .map_err(|e| MusicXServiceError { message: e.message })
    }

    pub fn recommend_songs_for_user(
        &self,
        user_id: &String,
        recommendation_algorithm: &dyn SongRecommendation,
    ) -> Result<Vec<Song>, MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        self.song_service
            .recommend_songs_for_user(user_id, recommendation_algorithm)
            .map_err(|e| MusicXServiceError { message: e.message })
    }

    pub fn add_song_to_user_playlist_for_user(
        &mut self,
        user_id: &String,
        playlist_id: &String,
        song_id: String,
    ) -> Result<(), MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        self.user_service
            .add_song_to_playlist(user_id, playlist_id, song_id.clone())
            .map_err(|e| MusicXServiceError { message: e.message })?;

        self.add_song_for_user(user_id, song_id)
            .map_err(|e| MusicXServiceError { message: e.message })
    }

    pub fn add_playlist_to_user(
        &mut self,
        user_id: &String,
        playlist: Playlist,
    ) -> Result<(), MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        for song_id in playlist.get_songs() {
            self.add_song_for_user(user_id, song_id.clone())
                .map_err(|e| MusicXServiceError { message: e.message })?;
        }

        self.user_service
            .add_playlist(user_id, playlist)
            .map_err(|e| MusicXServiceError { message: e.message })?;
        Ok(())
    }

    pub fn remove_playlist_from_user(
        &mut self,
        user_id: &String,
        playlist_id: &String,
    ) -> Result<(), MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        self.user_service
            .remove_playlist(user_id, playlist_id)
            .map_err(|e| MusicXServiceError { message: e.message })
    }

    pub fn remove_song_from_user_playlist(
        &mut self,
        user_id: &String,
        playlist_id: &String,
        song_id: String,
    ) -> Result<(), MusicXServiceError> {
        self.user_service
            .is_user_logged_in(user_id)
            .map_err(|_| MusicXServiceError {
                message: "User is not logged in".to_string(),
            })?;
        self.user_service
            .remove_song_from_playlist(user_id, playlist_id, song_id)
            .map_err(|e| MusicXServiceError { message: e.message })
    }
}
