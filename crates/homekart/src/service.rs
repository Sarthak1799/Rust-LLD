use crate::{
    filter::PropertyFilter,
    models::{
        booking::Booking,
        property::{Property, PropertyType},
        user::User,
    },
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct HomeKartService {
    pub users: HashMap<String, User>,
    pub properties: HashMap<String, Property>,
    pub bookings: HashMap<String, Booking>,
}

#[derive(Debug)]
pub enum HomeKartError {
    UserNotFound,
    PropertyNotFound,
    BookingNotFound,
}

impl HomeKartService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            properties: HashMap::new(),
            bookings: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, name: String) -> String {
        let id = generate_timestamp_uuid();
        let user = User::new(id.clone(), name);
        self.users.insert(id.clone(), user);
        id
    }

    pub fn add_property(
        &mut self,
        name: String,
        address: String,
        rent: f64,
        price: f64,
        rooms: i32,
        owner_id: String,
        kind: PropertyType,
    ) -> Result<String, HomeKartError> {
        let user = self
            .users
            .get_mut(&owner_id)
            .ok_or(HomeKartError::UserNotFound)?;

        let id = generate_timestamp_uuid();
        let property = Property::new(
            id.clone(),
            name,
            address,
            rent,
            price,
            rooms,
            owner_id.clone(),
            kind,
        );
        self.properties.insert(property.get_id().clone(), property);
        user.add_property(owner_id.clone());
        Ok(id)
    }

    pub fn add_booking(
        &mut self,
        property_id: String,
        user_id: String,
    ) -> Result<String, HomeKartError> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(HomeKartError::UserNotFound)?;
        let property = self
            .properties
            .get_mut(&property_id)
            .ok_or(HomeKartError::PropertyNotFound)?;
        let id = generate_timestamp_uuid();
        let booking = Booking::new(id.clone(), user_id.clone(), property_id.clone());
        self.bookings.insert(booking.get_id().clone(), booking);
        user.add_booking(id.clone());
        property.set_booking_id(id.clone());

        Ok(id)
    }

    pub fn list_properties(&self, filter: &dyn PropertyFilter) -> Vec<Property> {
        let properties: Vec<Property> = self
            .properties
            .iter()
            .filter(|(_, property)| !property.is_booked())
            .map(|(_, property)| property.clone())
            .collect();

        let properties = filter.filter_properties(properties);

        properties
    }

    pub fn list_bookings_for_user(&self, user_id: String) -> Result<Vec<Booking>, HomeKartError> {
        let user = self
            .users
            .get(&user_id)
            .ok_or(HomeKartError::UserNotFound)?;

        let bookings: Vec<Booking> = user
            .get_bookings()
            .iter()
            .filter_map(|booking_id| self.bookings.get(booking_id))
            .cloned()
            .collect();

        Ok(bookings)
    }

    pub fn cancel_booking(&mut self, booking_id: String) -> Result<(), HomeKartError> {
        let booking = self
            .bookings
            .remove(&booking_id)
            .ok_or(HomeKartError::BookingNotFound)?;

        let property = self
            .properties
            .get_mut(booking.get_property_id())
            .ok_or(HomeKartError::PropertyNotFound)?;

        let user = self
            .users
            .get_mut(booking.get_user_id())
            .ok_or(HomeKartError::UserNotFound)?;

        property.remove_booking_id();
        user.remove_booking(booking.get_id());
        self.bookings.remove(booking.get_id());
        Ok(())
    }
}

fn generate_timestamp_uuid() -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    uuid.to_string()
}
