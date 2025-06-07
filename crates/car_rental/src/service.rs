use std::collections::HashMap;

use crate::models::{
    car::{Car, CarMake},
    customer::Customer,
    reservation::Reservation,
};

pub struct Service {
    pub cars: HashMap<String, Car>,
    pub customers: HashMap<String, Customer>,
    pub reservations: HashMap<String, Reservation>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            cars: HashMap::new(),
            customers: HashMap::new(),
            reservations: HashMap::new(),
        }
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.insert(car.get_id().to_string(), car);
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.customers
            .insert(customer.get_id().to_string(), customer);
    }

    pub fn list_available_cars(
        &mut self,
        make: CarMake,
        start_date: &str,
        end_date: &str,
    ) -> Vec<&Car> {
        // First update availability for all cars matching the make
        for car in self.cars.values_mut() {
            if car.get_make() == &make {
                let car_id = car.get_id();
                let has_overlapping_reservation = self.reservations.values().any(|reservation| {
                    reservation.get_car_id() == car_id
                        && reservation.overlaps_with(start_date, end_date)
                });

                car.set_availability(!has_overlapping_reservation);
            }
        }

        // Then collect available cars of the requested make
        self.cars
            .values()
            .filter(|car| car.get_make() == &make && car.is_available())
            .collect()
    }

    pub fn add_reservation(&mut self, reservation: Reservation) {
        self.cars
            .get_mut(reservation.get_car_id())
            .map(|car| car.set_availability(false));
        self.reservations
            .insert(reservation.get_id().to_string(), reservation);
    }

    pub fn cancel_reservation(&mut self, reservation_id: &str) {
        if let Some(reservation) = self.reservations.remove(reservation_id) {
            if let Some(car) = self.cars.get_mut(reservation.get_car_id()) {
                car.set_availability(true);
            }
        }
    }

    pub fn get_car(&self, id: &str) -> Option<&Car> {
        self.cars.get(id)
    }

    pub fn get_customer(&self, id: &str) -> Option<&Customer> {
        self.customers.get(id)
    }

    pub fn get_reservation(&self, id: &str) -> Option<&Reservation> {
        self.reservations.get(id)
    }
}
