pub fn generate_random_id(prefix: &str) -> String {
    let id = uuid::Uuid::now_v7();
    format!("{}-{}", prefix, id.to_string())
}
