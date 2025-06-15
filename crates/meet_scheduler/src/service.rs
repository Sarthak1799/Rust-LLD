use std::collections::HashMap;

use crate::models::{calender::Calender, meeting::Meeting, user::User};

#[derive(Debug)]
pub struct Service {
    pub users: HashMap<String, User>,
    pub calender: Calender,
}

#[derive(Debug)]
pub enum ServiceError {
    UserNotFound,
    FailedToAddMeeting,
}

impl Service {
    pub fn new() -> Self {
        Service {
            users: HashMap::new(),
            calender: Calender::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_user_historty(&self, user_id: &str) -> Result<Vec<Meeting>, ServiceError> {
        let user = self.users.get(user_id).ok_or(ServiceError::UserNotFound)?;
        Ok(user.meetings.iter().cloned().collect::<Vec<_>>())
    }

    pub fn add_meeting_to_calender(
        &mut self,
        start_time: chrono::NaiveTime,
        end_time: chrono::NaiveTime,
        date: chrono::NaiveDate,
        participants: Vec<String>,
        title: String,
        description: String,
    ) -> Result<usize, ServiceError> {
        let meeting = self
            .calender
            .add_meeting(start_time, end_time, date, participants, title, description)
            .ok_or(ServiceError::FailedToAddMeeting)?;

        for user_id in &meeting.participants {
            let user = self
                .users
                .get_mut(user_id)
                .ok_or(ServiceError::UserNotFound)?;
            user.add_meeting(meeting.clone());
        }

        Ok(meeting.room)
    }
}

pub fn generate_random_id(prefix: &str) -> String {
    let id = uuid::Uuid::now_v7();
    format!("{}-{}", prefix, id.to_string())
}
