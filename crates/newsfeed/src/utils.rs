use uuid::Uuid;

pub fn generate_timestamp_uuid(prefix: &str) -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    format!("{}-{}", prefix, uuid.to_string())
}
