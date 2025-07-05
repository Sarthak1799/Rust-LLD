use limiter::{
    limiter_interface,
    rate_limiter::{Limiter, Request},
};
use uuid::Uuid;

fn main() {
    let algorithm = limiter_interface::SlidingWindowLimiter::new(5);
    let mut limiter = Limiter::new(Box::new(algorithm));

    let id1 = generate_timestamp_uuid();
    let id2 = generate_timestamp_uuid();
    let id3 = generate_timestamp_uuid();
    let id4 = generate_timestamp_uuid();
    let id5 = generate_timestamp_uuid();
    let id6 = generate_timestamp_uuid();

    // Example usage of the limiter
    let inputs = vec![
        Request::new(id1.clone(), "data1".to_string()),
        Request::new(id2.clone(), "data2".to_string()),
        Request::new(id3.clone(), "data3".to_string()),
        Request::new(id4.clone(), "data4".to_string()),
        Request::new(id5.clone(), "data5".to_string()),
        Request::new(id1.clone(), "data1".to_string()),
        Request::new(id6.clone(), "data6".to_string()),
        Request::new(id1.clone(), "data1".to_string()),
        Request::new(id2.clone(), "data2".to_string()),
        Request::new(id6.clone(), "data6".to_string()),
    ];

    for input in inputs {
        let _ = limiter
            .process(input.clone())
            .inspect_err(|e| println!("Error processing input: {:?}", e));
    }

    let _ = limiter
        .reset()
        .inspect_err(|e| println!("Error resetting limiter: {:?}", e));

    let new_algo = limiter_interface::CounterLimiter::new(2);
    limiter.set_limiter(Box::new(new_algo));

    // Example usage of the limiter
    let inputs = vec![
        Request::new(id1.clone(), "data1".to_string()),
        Request::new(id2.clone(), "data2".to_string()),
        Request::new(id3, "data3".to_string()),
        Request::new(id4, "data4".to_string()),
        Request::new(id5, "data5".to_string()),
        Request::new(id1.clone(), "data1".to_string()),
        Request::new(id6.clone(), "data6".to_string()),
        Request::new(id1, "data1".to_string()),
        Request::new(id2.clone(), "data2".to_string()),
        Request::new(id6.clone(), "data6".to_string()),
    ];

    for input in inputs {
        let _ = limiter
            .process(input.clone())
            .inspect_err(|e| println!("Error processing input: {:?}", e));
    }
}

fn generate_timestamp_uuid() -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    uuid.to_string()
}
