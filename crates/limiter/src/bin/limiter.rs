use std::{clone, sync::Arc};

use limiter::{
    limiter_interface,
    rate_limiter::{Limiter, Request},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let algorithm = limiter_interface::SlidingWindowLimiter::new(5);
    let limiter = Arc::new(Limiter::new(Box::new(algorithm)));

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

    let mut handle = Vec::new();
    for input in inputs {
        let clone_limiter = Arc::clone(&limiter);
        let crr = tokio::spawn(async move {
            let _ = clone_limiter
                .process(input.clone())
                .await
                .inspect_err(|e| println!("Error processing input: {:?}", e));
        });

        handle.push(crr);
    }

    futures::future::try_join_all(handle).await;

    println!("Testing with CounterLimiter...\n");

    let new_algo = limiter_interface::CounterLimiter::new(2);
    let limiter = Arc::new(Limiter::new(Box::new(new_algo)));

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

    let mut handle = Vec::new();
    for input in inputs {
        let clone_limiter = Arc::clone(&limiter);
        let crr = tokio::spawn(async move {
            let _ = clone_limiter
                .process(input.clone())
                .await
                .inspect_err(|e| println!("Error processing input: {:?}", e));
        });

        handle.push(crr);
    }
    futures::future::try_join_all(handle).await;
}

fn generate_timestamp_uuid() -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    uuid.to_string()
}
