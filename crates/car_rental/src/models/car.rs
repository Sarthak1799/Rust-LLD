#[derive(Debug, Clone)]
pub struct Car {
    pub id: String,
    pub make: CarMake,
    pub model: String,
    pub price_per_day: f64,
    pub is_available: bool,
}

impl Car {
    pub fn new(id: String, make: CarMake, model: String, price_per_day: f64) -> Self {
        Self {
            id,
            make,
            model,
            price_per_day,
            is_available: true,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_make(&self) -> &CarMake {
        &self.make
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_price_per_day(&self) -> f64 {
        self.price_per_day
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }
    pub fn set_availability(&mut self, available: bool) {
        self.is_available = available;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarMake {
    Suv,
    Sedan,
    Hatchback,
}
