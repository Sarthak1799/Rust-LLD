use cache_system::{
    cache::Cache,
    eveiction_policy::LRUBasedEviction,
    storage_interface::{HashMapStorage, StorageInterface},
};
fn main() {
    let storage = HashMapStorage::new();
    let eviction_policy = LRUBasedEviction::new();
    let mut cache = Cache::new(Box::new(storage), Box::new(eviction_policy), 2);

    cache
        .put("key1".to_string(), "value1".to_string())
        .expect("Failed to put value");
    cache
        .put("key2".to_string(), "value2".to_string())
        .expect("Failed to put value");
    let value = cache
        .retrieve(&"key1".to_string())
        .expect("Failed to retrieve value");

    println!("Retrieved value: {:?}", value);

    cache
        .put("key3".to_string(), "value3".to_string())
        .expect("Failed to put value");
    let value = cache
        .retrieve(&"key2".to_string())
        .expect("Failed to retrieve value");

    println!("Retrieved value after eviction: {:?}", value);
}
