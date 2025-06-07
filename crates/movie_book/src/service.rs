use std::collections::HashMap;

use crate::models::{
    booking::Booking, movie::Movie, seat::Seat, show::Show, theater::Theater, user::User,
};

pub struct Service {
    pub movies: HashMap<String, Movie>,
    pub users: HashMap<String, User>,
    pub theaters: HashMap<String, Theater>,
    pub bookings: HashMap<String, Booking>,
    pub movie_theater_map: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub enum ServiceError {
    MovieNotFound,
    UserNotFound,
    TheaterNotFound,
    BookingNotFound,
}

impl Service {
    pub fn new() -> Service {
        Service {
            movies: HashMap::new(),
            users: HashMap::new(),
            theaters: HashMap::new(),
            bookings: HashMap::new(),
            movie_theater_map: HashMap::new(),
        }
    }

    pub fn add_show_to_theater(
        &mut self,
        movie: Movie,
        theater_ids: Vec<&str>,
        start_time: String,
        end_time: String,
    ) -> Result<(), ServiceError> {
        for theater_id in theater_ids {
            let theater = self
                .theaters
                .get_mut(theater_id)
                .ok_or(ServiceError::TheaterNotFound)?;
            let show_id = format!("{}:{}:show", movie.get_id(), theater.get_id());
            let show = Show::new(
                show_id,
                movie.get_id().to_string(),
                theater.get_id().to_string(),
                start_time.clone(),
                end_time.clone(),
                vec![
                    Seat::new(
                        "1".to_string(),
                        1,
                        1,
                        true,
                        crate::models::seat::SeatCategory::Premium,
                    ),
                    Seat::default(),
                ],
            );
            self.movie_theater_map
                .entry(movie.get_id().to_string())
                .or_insert_with(Vec::new)
                .push(theater.get_id().to_string());
            theater.add_show(show);
        }

        self.movies.insert(movie.get_id().to_string(), movie);

        Ok(())
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.get_id().to_string(), user);
    }

    pub fn add_theater(&mut self, theater: Theater) {
        self.theaters.insert(theater.get_id().to_string(), theater);
    }

    pub fn add_booking(&mut self, booking: Booking) {
        self.bookings.insert(booking.get_id().to_string(), booking);
    }

    pub fn list_movies(&self) -> Vec<&Movie> {
        self.movies.values().collect()
    }

    pub fn list_theaters_for_movie(&self, movie_id: &str) -> Result<Vec<Theater>, ServiceError> {
        let theater_ids = self
            .movie_theater_map
            .get(movie_id)
            .ok_or(ServiceError::MovieNotFound)?;
        let theaters = theater_ids
            .iter()
            .filter_map(|id| self.theaters.get(id).cloned())
            .collect();
        Ok(theaters)
    }

    pub fn make_booking(
        &mut self,
        user_id: &str,
        show_id: &str,
        theater_id: &str,
    ) -> Result<String, ServiceError> {
        let user = self.users.get(user_id).ok_or(ServiceError::UserNotFound)?;
        let theater = self
            .theaters
            .get(theater_id)
            .ok_or(ServiceError::TheaterNotFound)?;
        let show = theater
            .get_shows()
            .iter()
            .find(|show| show.get_id() == show_id)
            .ok_or(ServiceError::MovieNotFound)?;

        let booking_id = format!("{}:{}:booking", user.get_id(), show_id);
        let total_price = show.seats.len() as f32 * 10.0; // Assuming each seat costs $10.0
        let booking = Booking::new(
            booking_id.clone(),
            user.get_id().to_string(),
            show.get_id().to_string(),
            show.get_seats()
                .iter()
                .map(|s| s.get_id().to_string())
                .collect(),
            total_price,
            theater.get_id().to_string(),
        );
        self.add_booking(booking);
        Ok(booking_id)
    }
}
