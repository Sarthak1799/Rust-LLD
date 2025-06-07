use crate::models::user::{Playlist, User};
use std::collections::HashMap;

#[derive(Debug)]
pub struct UserService {
    pub users: HashMap<String, User>,
    pub logged_in_users: Vec<String>, // List of logged-in user IDs
}

#[derive(Debug)]
pub struct UserServiceError {
    pub message: String,
}
impl UserServiceError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            logged_in_users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.get_id().clone(), user);
    }

    pub fn get_user(&self, id: &String) -> Option<&User> {
        self.users.get(id)
    }

    pub fn remove_user(&mut self, id: &String) {
        self.users.remove(id);
    }

    pub fn login(&mut self, id: &str, password: String) -> Result<(), UserServiceError> {
        let user = self.users.get(id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if user.get_password_hash() == &password {
            self.logged_in_users.push(id.to_string());
            Ok(())
        } else {
            Err(UserServiceError {
                message: "Invalid password".to_string(),
            })
        }
    }

    pub fn add_song_to_playlist(
        &mut self,
        user_id: &String,
        playlist_id: &String,
        song_id: String,
    ) -> Result<(), UserServiceError> {
        let user = self.users.get_mut(user_id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if !self.logged_in_users.contains(user_id) {
            return Err(UserServiceError {
                message: "User is not logged in".to_string(),
            });
        }

        user.add_song_to_playlist(playlist_id, song_id)
            .map_err(|e| UserServiceError { message: e })
    }

    pub fn add_playlist(
        &mut self,
        user_id: &String,
        playlist: Playlist,
    ) -> Result<(), UserServiceError> {
        let user = self.users.get_mut(user_id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if !self.logged_in_users.contains(user_id) {
            return Err(UserServiceError {
                message: "User is not logged in".to_string(),
            });
        }

        user.add_playlist(playlist);

        Ok(())
    }

    pub fn remove_playlist(
        &mut self,
        user_id: &String,
        playlist_id: &String,
    ) -> Result<(), UserServiceError> {
        let user = self.users.get_mut(user_id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if !self.logged_in_users.contains(user_id) {
            return Err(UserServiceError {
                message: "User is not logged in".to_string(),
            });
        }

        user.remove_playlist(playlist_id)
            .map_err(|e| UserServiceError { message: e })
    }
    pub fn get_playlists(&self, user_id: &String) -> Result<Vec<Playlist>, UserServiceError> {
        let user = self.users.get(user_id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if !self.logged_in_users.contains(user_id) {
            return Err(UserServiceError {
                message: "User is not logged in".to_string(),
            });
        }

        Ok(user.get_playlists().clone())
    }

    pub fn is_user_logged_in(&self, user_id: &String) -> Result<(), UserServiceError> {
        if self.logged_in_users.contains(user_id) {
            Ok(())
        } else {
            Err(UserServiceError {
                message: "User is not logged in".to_string(),
            })
        }
    }
    pub fn remove_song_from_playlist(
        &mut self,
        user_id: &String,
        playlist_id: &String,
        song_id: String,
    ) -> Result<(), UserServiceError> {
        let user = self.users.get_mut(user_id).ok_or(UserServiceError {
            message: "User not found".to_string(),
        })?;

        if !self.logged_in_users.contains(user_id) {
            return Err(UserServiceError {
                message: "User is not logged in".to_string(),
            });
        }

        user.remove_song_from_playlist(playlist_id, &song_id)
            .map_err(|e| UserServiceError { message: e })
    }
}
