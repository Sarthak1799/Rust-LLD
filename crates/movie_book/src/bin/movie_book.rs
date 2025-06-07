use movie_book::{
    models::{movie::Movie, theater::Theater, user::User},
    service::{self, Service},
};
fn main() {
    let mut service = Service::new();
    let movie = Movie::new(
        "1".to_string(),
        "Inception".to_string(),
        "Christopher Nolan".to_string(),
        2010,
    );
    let theater = Theater::new("1".to_string(), "AMC".to_string(), "New York".to_string());
    let theater2 = Theater::new(
        "2".to_string(),
        "Regal".to_string(),
        "Los Angeles".to_string(),
    );
    let theater3 = Theater::new(
        "3".to_string(),
        "Cinemark".to_string(),
        "Chicago".to_string(),
    );
    let user = User::new("1".to_string(), "John Doe".to_string(), "name".to_string());

    service.add_user(user);
    service.add_theater(theater);
    service.add_theater(theater2);
    service.add_theater(theater3);
    service
        .add_show_to_theater(movie, vec!["1", "2", "3"], "4".to_string(), "5".to_string())
        .expect("Failed to add show");

    let movies = service.list_movies();

    let movie = movies.first().expect("No movies found");
    println!("Movie ID: {}", movie.get_id());

    let theaters = service
        .list_theaters_for_movie(movie.get_id())
        .expect("No theaters found");

    for theater in &theaters {
        println!("Theater ID: {}", theater.get_id());
        println!("Theater Name: {}", theater.get_name());
        println!("Theater Location: {}", theater.get_location());
    }

    let thers = theaters.first().expect("No theaters found");

    let show = thers.get_shows().first().expect("No show found").get_id();

    let the_id = thers.get_id().to_string();

    let booking = service
        .make_booking("1", show, &the_id)
        .expect("Failed to make booking");
    println!("Booking ID: {}", booking);
}
