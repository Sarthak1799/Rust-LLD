use crate::service::generate_random_id;
use chrono::NaiveDate;
use std::{
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, HashMap},
};

pub const MAX_ROOMS: usize = 10;

use super::meeting::Meeting;

#[derive(Clone, Debug)]
pub struct MeetingEvent {
    pub rooms: Vec<usize>,
    pub meetings: BinaryHeap<Reverse<Meeting>>,
}

impl MeetingEvent {
    pub fn new() -> Self {
        MeetingEvent {
            rooms: Vec::new(),
            meetings: BinaryHeap::new(),
        }
    }

    pub fn add_meeting(&mut self, meeting: Meeting) {
        self.rooms.push(meeting.room);
        self.meetings.push(Reverse(meeting));
    }
}

#[derive(Debug)]
pub struct Calender {
    pub meetings: HashMap<chrono::NaiveDate, MeetingEvent>,
}

impl Calender {
    pub fn new() -> Self {
        Calender {
            meetings: HashMap::new(),
        }
    }

    pub fn add_meeting(
        &mut self,
        start_time: chrono::NaiveTime,
        end_time: chrono::NaiveTime,
        date: NaiveDate,
        participants: Vec<String>,
        title: String,
        description: String,
    ) -> Option<Meeting> {
        let exiting_date = self.meetings.entry(date).or_insert(MeetingEvent::new());
        while true {
            let top = exiting_date.meetings.peek();
            if let Some(meet) = top {
                if meet.0.end_time <= start_time {
                    exiting_date.rooms.retain(|&room| room != meet.0.room);
                    exiting_date.meetings.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        for room in 0..MAX_ROOMS {
            if !exiting_date.rooms.contains(&room) {
                let meeting = Meeting::new(
                    generate_random_id("meet"),
                    title.clone(),
                    description.clone(),
                    room,
                    date,
                    start_time,
                    end_time,
                    participants.clone(),
                );
                exiting_date.add_meeting(meeting.clone());
                return Some(meeting);
            }
        }

        None
    }
}
