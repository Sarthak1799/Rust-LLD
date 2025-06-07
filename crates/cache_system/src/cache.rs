use super::eveiction_policy::EvictionPolicy;
use super::storage_interface::StorageInterface;
pub struct Cache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    storage: Box<dyn StorageInterface<K, V>>,
    eviction_policy: Box<dyn EvictionPolicy<K>>,
    size: usize,
}

impl<K, V> Cache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new(
        storage: Box<dyn StorageInterface<K, V>>,
        eviction_policy: Box<dyn EvictionPolicy<K>>,
        size: usize,
    ) -> Self {
        Cache {
            storage,
            eviction_policy,
            size,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        self.storage
            .put(key.clone(), value)
            .map_err(|err| CacheError::InternalError(err.to_string()))?;
        self.eviction_policy
            .process(&key)
            .map_err(|err| CacheError::InternalError(err.to_string()))?;

        let current_size = self
            .storage
            .size()
            .map_err(|err| CacheError::InternalError(err.to_string()))?;

        if current_size > self.size {
            let to_evict = self
                .eviction_policy
                .evict()
                .map_err(|err| CacheError::InternalError(err.to_string()))?;

            to_evict
                .map(|key| -> Result<(), CacheError> {
                    self.storage
                        .delete(&key)
                        .map_err(|err| CacheError::InternalError(err.to_string()))?;

                    Ok(())
                })
                .transpose()?;
        }

        Ok(())
    }

    pub fn retrieve(&mut self, key: &K) -> Result<Option<V>, CacheError> {
        if !self.exists(key)? {
            return Err(CacheError::NotFound);
        }

        let value = self
            .storage
            .retrieve(key)
            .map_err(|err| CacheError::InternalError(err.to_string()))?;
        self.eviction_policy
            .process(key)
            .map_err(|err| CacheError::InternalError(err.to_string()))?;
        Ok(value)
    }

    pub fn delete(&mut self, key: &K) -> Result<(), CacheError> {
        self.storage
            .delete(key)
            .map_err(|err| CacheError::InternalError(err.to_string()))?;

        Ok(())
    }

    pub fn exists(&self, key: &K) -> Result<bool, CacheError> {
        self.storage
            .exists(key)
            .map_err(|err| CacheError::InternalError(err.to_string()))
    }
}

#[derive(Debug, Clone)]
pub enum CacheError {
    NotFound,
    AlreadyExists,
    InternalError(String),
}
impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::NotFound => write!(f, "Item not found"),
            CacheError::AlreadyExists => write!(f, "Item already exists"),
            CacheError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}
