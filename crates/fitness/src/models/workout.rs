pub trait WorkoutInterface {
    fn get_workout(&self) -> String;
    fn get_capacity(&self) -> u32;
    fn get_start_time(&self) -> String;
    fn get_end_time(&self) -> String;
    fn book_seat(&mut self) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct WeightLiftingWorkout {
    pub workout: String,
    pub capacity: u32,
    pub start_time: String,
    pub end_time: String,
}
impl WeightLiftingWorkout {
    pub fn new(workout: String, capacity: u32, start_time: String, end_time: String) -> Self {
        Self {
            workout,
            capacity,
            start_time,
            end_time,
        }
    }
}

impl WorkoutInterface for WeightLiftingWorkout {
    fn get_workout(&self) -> String {
        self.workout.clone()
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }

    fn get_start_time(&self) -> String {
        self.start_time.clone()
    }

    fn get_end_time(&self) -> String {
        self.end_time.clone()
    }
    fn book_seat(&mut self) -> Result<(), String> {
        if self.get_capacity() > 0 {
            self.capacity -= 1;
            Ok(())
        } else {
            Err("No available seats".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardioWorkout {
    pub workout: String,
    pub capacity: u32,
    pub start_time: String,
    pub end_time: String,
}
impl CardioWorkout {
    pub fn new(workout: String, capacity: u32, start_time: String, end_time: String) -> Self {
        Self {
            workout,
            capacity,
            start_time,
            end_time,
        }
    }
}
impl WorkoutInterface for CardioWorkout {
    fn get_workout(&self) -> String {
        self.workout.clone()
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }

    fn get_start_time(&self) -> String {
        self.start_time.clone()
    }

    fn get_end_time(&self) -> String {
        self.end_time.clone()
    }

    fn book_seat(&mut self) -> Result<(), String> {
        if self.get_capacity() > 0 {
            self.capacity -= 1;
            Ok(())
        } else {
            Err("No available seats".to_string())
        }
    }
}
