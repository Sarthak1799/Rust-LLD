#[derive(Debug, Clone, Default)]
pub struct Seat {
    pub id: String,
    pub row: i32,
    pub column: i32,
    pub is_available: bool,
    pub category: SeatCategory,
}
impl Seat {
    pub fn new(
        id: String,
        row: i32,
        column: i32,
        is_available: bool,
        category: SeatCategory,
    ) -> Seat {
        Seat {
            id,
            row,
            column,
            is_available,
            category,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }

    pub fn get_category(&self) -> &SeatCategory {
        &self.category
    }
    pub fn set_availability(&mut self, is_available: bool) {
        self.is_available = is_available;
    }
}

#[derive(Debug, Clone, Default)]
pub enum SeatCategory {
    #[default]
    Regular,
    Premium,
    VIP,
}
