use std::{collections::HashMap, sync::Arc};

use super::workout::WorkoutInterface;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub workouts: HashMap<String, Vec<String>>,
}

impl User {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            workouts: HashMap::new(),
        }
    }

    pub fn add_workout(&mut self, date: String, workout_id: String) {
        self.workouts
            .entry(date)
            .or_insert_with(Vec::new)
            .push(workout_id);
    }

    pub fn get_workout(&self, date: &str) -> Option<Vec<String>> {
        self.workouts.get(date).cloned()
    }
}
