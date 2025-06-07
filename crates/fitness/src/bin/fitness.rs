use fitness::{
    models::{
        user::User,
        workout::{self, WeightLiftingWorkout},
    },
    service::FitnessService,
};
fn main() {
    // Initialize the fitness service
    let mut fitness_service = FitnessService::new();

    // Example usage
    let user_id = "user123";
    let workout_id = "Weight Lifting";
    let date = "20-10-2025";

    // Add a user
    let user = User::new(
        user_id.to_string(),
        "John Doe".to_string(),
        "some@some".to_string(),
    );
    fitness_service.add_user(user);
    let workout1 = WeightLiftingWorkout::new(
        "Weight Lifting".to_string(),
        10,
        "1".to_string(),
        "2".to_string(),
    );
    let workout2 =
        WeightLiftingWorkout::new("Cardio".to_string(), 5, "2".to_string(), "3".to_string());

    fitness_service.add_workout_to_center(
        "center1".to_string(),
        Box::new(workout1),
        "01-10-2025".to_string(),
        "20-10-2025".to_string(),
    );
    fitness_service.add_workout_to_center(
        "center2".to_string(),
        Box::new(workout2),
        "02-09-2025".to_string(),
        "20-10-2025".to_string(),
    );

    let workouts = fitness_service.get_all_workouts();
    for (center_id, workouts) in workouts {
        println!("Center ID: {}", center_id);
        for workout in workouts {
            println!(
                "Workout: {}, Capacity: {}, Start Time: {}, End Time: {}, Start Date: {}, End Date: {}",
                workout.get_workout_ref().get_workout(),
                workout.get_workout_ref().get_capacity(),
                workout.get_workout_ref().get_start_time(),
                workout.get_workout_ref().get_end_time(),
                workout.start_date,
                workout.end_date
            );
        }
    }

    fitness_service
        .book_workout_for_user(user_id, "center1", workout_id, date)
        .expect("Failed to book workout");

    let user_workouts = fitness_service
        .list_workouts_for_user(user_id, date)
        .expect("Failed to list workouts for user");

    for workout in user_workouts {
        println!(
            "User {} has booked workout: {} on date: {}",
            user_id, workout, date
        );
    }

    let workouts = fitness_service.get_all_workouts();
    for (center_id, workouts) in workouts {
        println!("Center ID: {}", center_id);
        for workout in workouts {
            println!(
                "Workout: {}, Capacity: {}, Start Time: {}, End Time: {}, Start Date: {}, End Date: {}",
                workout.get_workout_ref().get_workout(),
                workout.get_workout_ref().get_capacity(),
                workout.get_workout_ref().get_start_time(),
                workout.get_workout_ref().get_end_time(),
                workout.start_date,
                workout.end_date
            );
        }
    }
}
