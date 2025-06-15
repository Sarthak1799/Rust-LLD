use meet_scheduler::models::user::User;

use meet_scheduler::service::{generate_random_id, Service};
fn main() {
    let id1 = generate_random_id("user");
    let id2 = generate_random_id("user");
    let id3 = generate_random_id("user");
    let id4 = generate_random_id("user");
    let user1 = User::new(id1.clone(), "Alice".to_string());
    let user2 = User::new(id2.clone(), "Bob".to_string());
    let user3 = User::new(id3.clone(), "Charlie".to_string());
    let user4 = User::new(id4.clone(), "David".to_string());
    let mut service = Service::new();
    service.add_user(user1);
    service.add_user(user2);
    service.add_user(user3);
    service.add_user(user4);
    let start_time = chrono::NaiveTime::parse_from_str("10:00:00", "%H:%M:%S")
        .expect("failed to parse start time");
    let end_time = chrono::NaiveTime::parse_from_str("11:00:00", "%H:%M:%S")
        .expect("failed to parse end time");
    let date = chrono::NaiveDate::from_ymd_opt(2025, 10, 01).expect("failed to create date");
    let participants = vec![id1.clone(), id2.clone()];
    let title = "Team Meeting".to_string();
    let description = "Discuss project updates".to_string();
    let room = service
        .add_meeting_to_calender(start_time, end_time, date, participants, title, description)
        .expect("failed to add meeting to calendar");
    println!("Meeting scheduled in room: {:?}", room);

    let start_time = chrono::NaiveTime::parse_from_str("11:30:00", "%H:%M:%S")
        .expect("failed to parse start time");
    let end_time = chrono::NaiveTime::parse_from_str("12:30:00", "%H:%M:%S")
        .expect("failed to parse end time");
    let date = chrono::NaiveDate::from_ymd_opt(2025, 10, 01).expect("failed to create date");
    let participants = vec![id1.clone(), id4];
    let title = "Team Meeting".to_string();
    let description = "Discuss project updates".to_string();
    let room = service
        .add_meeting_to_calender(start_time, end_time, date, participants, title, description)
        .expect("failed to add meeting to calendar");
    println!("Meeting scheduled in room: {:?}", room);

    let meetings = service
        .get_user_historty(&id1)
        .expect(" failed to get user history");
    for meeting in meetings {
        println!(
            "Meeting for user {}: {:?}",
            id1,
            format!(
                "Room: {}, Start: {}, End: {}, Date: {}, Participants: {:?}, Title: {}, Description: {}",
                meeting.room,
                meeting.start_time,
                meeting.end_time,
                meeting.date,
                meeting.participants,
                meeting.title,
                meeting.description
            )
        );
    }
}
