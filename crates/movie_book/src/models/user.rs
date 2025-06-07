pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
}
impl User {
    pub fn new(id: String, username: String, name: String) -> User {
        User { id, username, name }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
