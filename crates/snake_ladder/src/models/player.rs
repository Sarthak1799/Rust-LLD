#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: String,
}

impl Player {
    pub fn new(name: String, id: String) -> Self {
        Player { name, id }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
}
