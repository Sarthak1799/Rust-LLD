use uuid;

pub fn generate_uuid(prefix: &str) -> String {
    format!("{}-{}", prefix, uuid::Uuid::now_v7().to_string())
}
