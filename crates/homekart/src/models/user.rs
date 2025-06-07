#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub property_ids: Vec<String>,
    pub booking_ids: Vec<String>,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            property_ids: Vec::new(),
            booking_ids: Vec::new(),
        }
    }

    pub fn add_property(&mut self, property_id: String) {
        self.property_ids.push(property_id);
    }

    pub fn add_booking(&mut self, booking_id: String) {
        self.booking_ids.push(booking_id);
    }

    pub fn remove_property(&mut self, property_id: &String) {
        self.property_ids.retain(|id| id != property_id);
    }
    pub fn remove_booking(&mut self, booking_id: &String) {
        self.booking_ids.retain(|id| id != booking_id);
    }
    pub fn get_properties(&self) -> &Vec<String> {
        &self.property_ids
    }
    pub fn get_bookings(&self) -> &Vec<String> {
        &self.booking_ids
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
}
