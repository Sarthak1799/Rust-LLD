use crate::models::meeting::Meeting;

#[derive(Debug)]
pub struct User {
    pub id: String,   // Unique identifier for the user
    pub name: String, // Name of the user
    pub meetings: Vec<Meeting>,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        User {
            id,
            name,
            meetings: Vec::new(),
        }
    }

    pub fn add_meeting(&mut self, meeting: Meeting) {
        self.meetings.push(meeting);
    }

    pub fn remove_meeting(&mut self, meeting_id: &str) {
        self.meetings.retain(|m| m.id != meeting_id);
    }

    pub fn get_meetings(&self) -> &Vec<Meeting> {
        &self.meetings
    }
}
