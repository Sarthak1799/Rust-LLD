use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{comment::Comment, post::Post, user::User};

const MAX_FEED_SIZE: usize = 10;

#[derive(Debug)]
pub enum ServiceError {
    UserNotFound,
    PostNotFound,
    UserNotLoggedIn,
    GenericError(&'static str),
}

#[derive(Debug)]
pub struct Service {
    pub users: HashMap<String, User>,
    pub logged_in_users: HashSet<String>,
    pub posts: HashMap<String, Post>,
    pub feed_map: HashMap<String, VecDeque<String>>, // user_id -> [post_id]
    pub followers_map: HashMap<String, Vec<String>>, // user_id -> followers
}

impl Service {
    pub fn new() -> Self {
        Service {
            users: HashMap::new(),
            logged_in_users: HashSet::new(),
            posts: HashMap::new(),
            feed_map: HashMap::new(),
            followers_map: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.get_id().to_string(), user);
    }

    pub fn add_post(&mut self, post: Post) -> Result<(), ServiceError> {
        let user_id = post.get_user_id().to_string();
        if !self.logged_in_users.contains(&user_id) {
            return Err(ServiceError::UserNotLoggedIn);
        }

        let post_id = post.get_id().to_string();

        self.posts.insert(post.get_id().to_string(), post.clone());
        self.feed_map
            .entry(user_id.clone())
            .or_default()
            .push_back(post_id.clone());
        let size = self
            .feed_map
            .get(&user_id)
            .ok_or(ServiceError::GenericError("Feed not found"))?
            .len();
        if size > MAX_FEED_SIZE {
            self.feed_map
                .get_mut(&user_id)
                .ok_or(ServiceError::GenericError("Feed not found"))?
                .pop_front(); // Remove the oldest post if feed exceeds max size
        }

        let followers = self
            .followers_map
            .get(&user_id)
            .cloned()
            .unwrap_or_default();
        for follower in followers {
            self.feed_map
                .entry(follower.clone())
                .or_default()
                .push_back(post_id.clone());
            let size = self
                .feed_map
                .get(&follower)
                .ok_or(ServiceError::GenericError("Feed not found"))?
                .len();

            if size > MAX_FEED_SIZE {
                self.feed_map
                    .get_mut(&follower)
                    .ok_or(ServiceError::GenericError("Feed not found"))?
                    .pop_front(); // Remove the oldest post if feed exceeds max size
            }
        }

        Ok(())
    }

    pub fn get_user(&self, user_id: &str) -> Result<&User, ServiceError> {
        self.users.get(user_id).ok_or(ServiceError::UserNotFound)
    }

    pub fn login_user(&mut self, user_id: &str) -> Result<(), ServiceError> {
        let user = self.users.get(user_id).ok_or(ServiceError::UserNotFound)?;
        self.logged_in_users.insert(user.get_id().to_string());
        Ok(())
    }

    pub fn logout_user(&mut self, user_id: &str) -> Result<(), ServiceError> {
        if self.logged_in_users.remove(user_id) {
            Ok(())
        } else {
            Err(ServiceError::UserNotLoggedIn)
        }
    }

    pub fn add_comment(&mut self, post_id: &str, comment: Comment) -> Result<(), ServiceError> {
        let user_id = comment.get_user_id().to_string();
        if !self.logged_in_users.contains(&user_id) {
            return Err(ServiceError::UserNotLoggedIn);
        }

        let post = self
            .posts
            .get_mut(post_id)
            .ok_or(ServiceError::PostNotFound)?;
        post.add_comment(comment);
        Ok(())
    }

    pub fn get_feed(&self, user_id: &str) -> Result<Vec<Post>, ServiceError> {
        if !self.logged_in_users.contains(user_id) {
            return Err(ServiceError::UserNotLoggedIn);
        }

        let post_ids = self
            .feed_map
            .get(user_id)
            .ok_or(ServiceError::GenericError("Feed not found"))?;

        let mut feed_posts = Vec::new();
        for post_id in post_ids.iter() {
            if let Some(post) = self.posts.get(post_id) {
                feed_posts.push(post.clone());
            }
        }

        Ok(feed_posts)
    }

    pub fn follow_user(&mut self, user_id: &str, follower_id: &str) -> Result<(), ServiceError> {
        if !self.users.contains_key(user_id) || !self.users.contains_key(follower_id) {
            return Err(ServiceError::UserNotFound);
        }

        let followers = self.followers_map.entry(user_id.to_string()).or_default();
        if !followers.contains(&follower_id.to_string()) {
            followers.push(follower_id.to_string());
        }

        Ok(())
    }

    pub fn unfollow_user(&mut self, user_id: &str, follower_id: &str) -> Result<(), ServiceError> {
        if !self.users.contains_key(user_id) || !self.users.contains_key(follower_id) {
            return Err(ServiceError::UserNotFound);
        }

        if let Some(followers) = self.followers_map.get_mut(user_id) {
            if let Some(pos) = followers.iter().position(|x| x == follower_id) {
                followers.remove(pos);
            }
        }

        Ok(())
    }
}
