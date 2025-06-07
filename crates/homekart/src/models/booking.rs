#[derive(Debug, Clone)]
pub struct Booking {
    pub id: String,
    pub user_id: String,
    pub property_id: String,
}

impl Booking {
    pub fn new(id: String, user_id: String, property_id: String) -> Self {
        Self {
            id,
            user_id,
            property_id,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_property_id(&self) -> &String {
        &self.property_id
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }
}
