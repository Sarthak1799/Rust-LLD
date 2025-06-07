#[derive(Debug, Clone)]
pub struct Property {
    pub id: String,
    pub name: String,
    pub address: String,
    pub rent: f64,
    pub price: f64,
    pub rooms: i32,
    pub owner_id: String,
    pub kind: PropertyType,
    pub booking_id: Option<String>,
}

impl Property {
    pub fn new(
        id: String,
        name: String,
        address: String,
        rent: f64,
        price: f64,
        rooms: i32,
        owner_id: String,
        kind: PropertyType,
    ) -> Self {
        Self {
            id,
            name,
            address,
            rent,
            price,
            rooms,
            owner_id,
            kind,
            booking_id: None,
        }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_address(&self) -> &String {
        &self.address
    }
    pub fn get_rent(&self) -> f64 {
        self.rent
    }
    pub fn get_price(&self) -> f64 {
        self.price
    }
    pub fn get_rooms(&self) -> i32 {
        self.rooms
    }
    pub fn get_owner_id(&self) -> &String {
        &self.owner_id
    }
    pub fn get_kind(&self) -> &PropertyType {
        &self.kind
    }
    pub fn get_booking_id(&self) -> Option<&String> {
        self.booking_id.as_ref()
    }
    pub fn set_booking_id(&mut self, booking_id: String) {
        self.booking_id = Some(booking_id);
    }
    pub fn remove_booking_id(&mut self) {
        self.booking_id = None;
    }
    pub fn is_booked(&self) -> bool {
        self.booking_id.is_some()
    }
}

#[derive(Debug, Clone)]
pub enum PropertyType {
    Apartment,
    House,
    Villa,
    Studio,
}
