use std::collections::HashMap;

pub trait StorageInterface<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    /// Saves a value in the storage with the given key.
    fn put(&mut self, key: K, value: V) -> Result<(), StorageError>;

    /// Retrieves a value from the storage by its key.
    fn retrieve(&self, key: &K) -> Result<Option<V>, StorageError>;

    /// Deletes a value from the storage by its key.
    fn delete(&mut self, key: &K) -> Result<(), StorageError>;

    /// Checks if a key exists in the storage.
    fn exists(&self, key: &K) -> Result<bool, StorageError>;

    /// get size of the storage
    fn size(&self) -> Result<usize, StorageError>;
}

#[derive(Debug, Clone)]
pub enum StorageError {
    NotFound,
    AlreadyExists,
    InternalError(String),
}
impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NotFound => write!(f, "Item not found"),
            StorageError::AlreadyExists => write!(f, "Item already exists"),
            StorageError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub struct HashMapStorage<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    pub store: HashMap<K, V>,
}

impl<K, V> HashMapStorage<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl<K, V> StorageInterface<K, V> for HashMapStorage<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn put(&mut self, key: K, value: V) -> Result<(), StorageError> {
        if self.store.contains_key(&key) {
            return Err(StorageError::AlreadyExists);
        }
        self.store.insert(key, value);
        Ok(())
    }

    fn retrieve(&self, key: &K) -> Result<Option<V>, StorageError> {
        Ok(self.store.get(key).cloned())
    }

    fn delete(&mut self, key: &K) -> Result<(), StorageError> {
        if !self.store.contains_key(key) {
            return Err(StorageError::NotFound);
        }

        self.store.remove(key);
        Ok(())
    }

    fn exists(&self, key: &K) -> Result<bool, StorageError> {
        Ok(self.store.contains_key(key))
    }

    fn size(&self) -> Result<usize, StorageError> {
        Ok(self.store.len())
    }
}
