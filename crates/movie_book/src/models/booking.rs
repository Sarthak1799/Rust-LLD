pub struct Booking {
    pub id: String,
    pub user_id: String,
    pub show_id: String,
    pub seats: Vec<String>,
    pub total_price: f32,
    pub theater_id: String,
}

impl Booking {
    pub fn new(
        id: String,
        user_id: String,
        show_id: String,
        seats: Vec<String>,
        total_price: f32,
        theater_id: String,
    ) -> Booking {
        Booking {
            id,
            user_id,
            show_id,
            seats,
            total_price,
            theater_id,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_show_id(&self) -> &str {
        &self.show_id
    }

    pub fn get_seats(&self) -> &Vec<String> {
        &self.seats
    }

    pub fn get_total_price(&self) -> f32 {
        self.total_price
    }

    pub fn get_theater_id(&self) -> &str {
        &self.theater_id
    }
}
