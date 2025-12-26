#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

impl Comment {
    pub fn new(post_id: String, user_id: String, content: String) -> Self {
        let id = crate::utils::generate_timestamp_uuid("comment");
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string();
        Comment {
            id,
            post_id,
            user_id,
            content,
            created_at,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_post_id(&self) -> &str {
        &self.post_id
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
}
