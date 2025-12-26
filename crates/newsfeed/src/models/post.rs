use crate::{models::comment::Comment, utils};

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
    pub comments: Vec<Comment>,
}

impl Post {
    pub fn new(user_id: String, content: String) -> Self {
        let id = utils::generate_timestamp_uuid("post");
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string();
        Post {
            id,
            user_id,
            content,
            created_at,
            comments: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }

    pub fn add_comment(&mut self, comment_id: Comment) {
        self.comments.push(comment_id);
    }
    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }
}
