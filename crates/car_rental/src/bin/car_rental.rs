use car_rental::{
    models::{
        car::{Car, CarMake},
        customer::Customer,
        reservation::Reservation,
    },
    service::Service,
};
use uuid::Uuid;

fn main() {
    let mut service = Service::new();
    let customer1_id = generate_timestamp_uuid();
    let customer2_id = generate_timestamp_uuid();
    let car1_id = generate_timestamp_uuid();
    let car2_id = generate_timestamp_uuid();
    let reservation_id = generate_timestamp_uuid();
    let customer1 = Customer::new(
        customer1_id.clone(),
        "John Doe".to_string(),
        "sometjing".to_string(),
        "4654".to_string(),
        "some_lisence".to_string(),
    );
    service.add_customer(customer1);
    let customer2 = Customer::new(
        customer2_id.clone(),
        "Jane Smith".to_string(),
        "sometjing".to_string(),
        "4654".to_string(),
        "some_lisence".to_string(),
    );
    service.add_customer(customer2);
    let car1 = Car::new(car1_id.clone(), CarMake::Sedan, "Camry".to_string(), 50.0);
    service.add_car(car1);
    let car2 = Car::new(car2_id.clone(), CarMake::Suv, "RAV4".to_string(), 70.0);
    service.add_car(car2);

    let available_cars = service.list_available_cars(CarMake::Sedan, "2023-10-01", "2023-10-10");
    for car in available_cars {
        println!("Available Car: {} - {}", car.get_id(), car.get_model());
    }

    let reservation = Reservation::new(
        reservation_id.clone(),
        customer1_id.to_string(),
        car1_id.to_string(),
        "2023-10-01".to_string(),
        "2023-10-10".to_string(),
    );
    service.add_reservation(reservation);

    let available_cars = service.list_available_cars(CarMake::Sedan, "2023-10-01", "2023-10-10");
    for car in available_cars {
        println!("Available Car: {} - {}", car.get_id(), car.get_model());
    }

    let available_cars = service.list_available_cars(CarMake::Suv, "2023-10-01", "2023-10-10");
    for car in available_cars {
        println!("Available Car: {} - {}", car.get_id(), car.get_model());
    }

    service.cancel_reservation(&reservation_id);

    let available_cars = service.list_available_cars(CarMake::Sedan, "2023-10-01", "2023-10-10");
    for car in available_cars {
        println!("Available Car: {} - {}", car.get_id(), car.get_model());
    }
}

fn generate_timestamp_uuid() -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    uuid.to_string()
}
