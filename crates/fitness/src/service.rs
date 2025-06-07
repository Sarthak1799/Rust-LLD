use std::{collections::HashMap, sync::Arc};

use crate::models::{user::User, workout::WorkoutInterface};

pub struct FitnessService {
    pub users: HashMap<String, User>,
    pub centers: HashMap<String, Vec<WorkoutSchedule>>,
}

#[derive(Debug)]
pub enum ServiceError {
    UserNotFound,
    WorkoutNotFound,
    InvalidDate,
}

impl FitnessService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            centers: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn add_workout_to_center(
        &mut self,
        center_id: String,
        workout: Box<dyn WorkoutInterface>,
        start_date: String,
        end_date: String,
    ) {
        self.centers
            .entry(center_id)
            .or_insert_with(Vec::new)
            .push(WorkoutSchedule::new(start_date, end_date, workout));
    }

    pub fn get_workouts_in_center(&self, center_id: &str) -> Option<&Vec<WorkoutSchedule>> {
        self.centers.get(center_id)
    }

    pub fn get_all_workouts(&self) -> Vec<(String, &Vec<WorkoutSchedule>)> {
        let mut all_workouts = Vec::new();
        for (center_id, workouts) in &self.centers {
            all_workouts.push((center_id.clone(), workouts));
        }
        all_workouts
    }

    pub fn book_workout_for_user(
        &mut self,
        user_id: &str,
        center_id: &str,
        workout_id: &str,
        date: &str,
    ) -> Result<(), ServiceError> {
        let user = self
            .users
            .get_mut(user_id)
            .ok_or(ServiceError::UserNotFound)?;

        let workouts = self
            .centers
            .get_mut(center_id)
            .ok_or(ServiceError::WorkoutNotFound)?;

        for schedule in workouts {
            // Check if this is the correct workout and date is valid
            if schedule.get_workout().get_workout() == workout_id {
                // Parse all dates to compare them
                let workout_date = chrono::NaiveDate::parse_from_str(date, "%d-%m-%Y")
                    .map_err(|_| ServiceError::InvalidDate)?;
                let start_date =
                    chrono::NaiveDate::parse_from_str(&schedule.get_start_date(), "%d-%m-%Y")
                        .map_err(|_| ServiceError::InvalidDate)?;
                let end_date =
                    chrono::NaiveDate::parse_from_str(&schedule.get_end_date(), "%d-%m-%Y")
                        .map_err(|_| ServiceError::InvalidDate)?;

                // Check if date falls within the schedule range
                if workout_date >= start_date && workout_date <= end_date {
                    user.add_workout(date.to_string(), schedule.get_workout().get_workout());
                    schedule
                        .get_workout()
                        .book_seat()
                        .map_err(|_| ServiceError::WorkoutNotFound)?;
                    return Ok(());
                } else {
                    return Err(ServiceError::InvalidDate);
                }
            }
        }

        Err(ServiceError::WorkoutNotFound)
    }

    pub fn list_workouts_for_user(
        &self,
        user_id: &str,
        date: &str,
    ) -> Result<Vec<String>, ServiceError> {
        let user = self.users.get(user_id).ok_or(ServiceError::UserNotFound)?;

        let mut workout_names = Vec::new();
        for (date_key, workout_ids) in &user.workouts {
            if date_key == date {
                workout_names.extend(workout_ids.clone());
            }
        }

        if workout_names.is_empty() {
            Err(ServiceError::WorkoutNotFound)
        } else {
            Ok(workout_names)
        }
    }
}

pub struct WorkoutSchedule {
    pub start_date: String,
    pub end_date: String,
    pub workout: Box<dyn WorkoutInterface>,
}

impl WorkoutSchedule {
    pub fn new(start_date: String, end_date: String, workout: Box<dyn WorkoutInterface>) -> Self {
        Self {
            start_date,
            end_date,
            workout,
        }
    }
}
impl WorkoutSchedule {
    pub fn get_start_date(&self) -> String {
        self.start_date.clone()
    }

    pub fn get_end_date(&self) -> String {
        self.end_date.clone()
    }

    pub fn get_workout(&mut self) -> &mut dyn WorkoutInterface {
        self.workout.as_mut()
    }

    pub fn get_workout_ref(&self) -> &dyn WorkoutInterface {
        self.workout.as_ref()
    }
}
