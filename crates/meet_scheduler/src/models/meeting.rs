#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub description: String,
    pub room: usize,                   // Room ID where the meeting will take place
    pub date: chrono::NaiveDate,       // Date of the meeting in ISO 8601 format
    pub start_time: chrono::NaiveTime, // ISO 8601 format
    pub end_time: chrono::NaiveTime,   // ISO 8601 format
    pub participants: Vec<String>,     // List of participant IDs
}

impl Ord for Meeting {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_time.cmp(&other.end_time)
    }
}
impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Meeting {
    pub fn new(
        id: String,
        title: String,
        description: String,
        room: usize,
        date: chrono::NaiveDate,
        start_time: chrono::NaiveTime,
        end_time: chrono::NaiveTime,
        participants: Vec<String>,
    ) -> Self {
        Meeting {
            id,
            title,
            description,
            room,
            date,
            start_time,
            end_time,
            participants,
        }
    }
}
