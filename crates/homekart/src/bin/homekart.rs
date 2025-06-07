use homekart::{
    filter::{PriceBasedFilter, RommBasedFilter},
    models::property::PropertyType,
    service::HomeKartService,
};

fn main() {
    let mut service = HomeKartService::new();
    let user1_id = service.add_user("John Doe".to_string());
    let user2_id = service.add_user("Jane Smith".to_string());
    let property1_id = service
        .add_property(
            "Regular Apartment".to_string(),
            "128 Main St".to_string(),
            1500.0,
            300000.0,
            3,
            user1_id.clone(),
            PropertyType::Apartment,
        )
        .expect("Failed to add property");
    let property2_id = service
        .add_property(
            "Luxury Apartment".to_string(),
            "123 Main St".to_string(),
            3500.0,
            900000.0,
            2,
            user2_id.clone(),
            PropertyType::Villa,
        )
        .expect("Failed to add property");

    let properties = service.list_properties(&RommBasedFilter);
    for property in properties {
        println!(
            "Property ID: {}, Name: {}, Address: {}, Rent: {}, Price: {}, Rooms: {}, Owner ID: {}",
            property.get_id(),
            property.get_name(),
            property.get_address(),
            property.get_rent(),
            property.get_price(),
            property.get_rooms(),
            property.get_owner_id()
        );
    }

    let booking_id = service
        .add_booking(property1_id.clone(), user1_id.clone())
        .expect("Failed to book property");

    let user_bookings = service
        .list_bookings_for_user(user1_id.clone())
        .expect("Failed to list bookings");
    for booking in user_bookings {
        println!(
            "Booking ID: {}, Property ID: {}, User ID: {}",
            booking.get_id(),
            booking.get_property_id(),
            booking.get_user_id()
        );
    }

    let properties = service.list_properties(&PriceBasedFilter);
    for property in properties {
        println!(
            "Property ID: {}, Name: {}, Address: {}, Rent: {}, Price: {}, Rooms: {}, Owner ID: {}",
            property.get_id(),
            property.get_name(),
            property.get_address(),
            property.get_rent(),
            property.get_price(),
            property.get_rooms(),
            property.get_owner_id()
        );
    }
}
