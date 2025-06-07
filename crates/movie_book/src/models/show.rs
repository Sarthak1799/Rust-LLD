use super::seat::{self, Seat};

#[derive(Debug, Clone)]
pub struct Show {
    pub id: String,
    pub movie_id: String,
    pub theater_id: String,
    pub start_time: String,
    pub end_time: String,
    pub seats: Vec<Seat>,
}

impl Show {
    pub fn new(
        id: String,
        movie_id: String,
        theater_id: String,
        start_time: String,
        end_time: String,
        seats: Vec<Seat>,
    ) -> Show {
        Show {
            id,
            movie_id,
            theater_id,
            start_time,
            end_time,
            seats,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_movie_id(&self) -> &str {
        &self.movie_id
    }

    pub fn get_theater_id(&self) -> &str {
        &self.theater_id
    }

    pub fn get_start_time(&self) -> &str {
        &self.start_time
    }

    pub fn get_end_time(&self) -> &str {
        &self.end_time
    }

    pub fn get_seats(&self) -> &Vec<Seat> {
        &self.seats
    }
}
