use crate::utils::generate_uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    username: String,
    email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        let id = generate_uuid("user");
        Self {
            id,
            username,
            email,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }
}
