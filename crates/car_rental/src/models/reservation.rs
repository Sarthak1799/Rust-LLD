#[derive(Debug, Clone)]
pub struct Reservation {
    pub id: String,
    pub customer_id: String,
    pub car_id: String,
    pub start_date: String,
    pub end_date: String,
}
impl Reservation {
    pub fn new(
        id: String,
        customer_id: String,
        car_id: String,
        start_date: String,
        end_date: String,
    ) -> Self {
        Self {
            id,
            customer_id,
            car_id,
            start_date,
            end_date,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn get_car_id(&self) -> &str {
        &self.car_id
    }

    pub fn get_start_date(&self) -> &str {
        &self.start_date
    }

    pub fn get_end_date(&self) -> &str {
        &self.end_date
    }

    pub fn overlaps_with(&self, start_date: &str, end_date: &str) -> bool {
        (self.start_date.as_str() <= end_date) && (start_date <= self.end_date.as_str())
    }
}
