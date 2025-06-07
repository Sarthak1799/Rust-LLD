use std::collections::{HashMap, LinkedList};

pub trait EvictionPolicy<K>
where
    K: Clone + Eq + std::hash::Hash,
{
    fn process(&mut self, key: &K) -> Result<(), EvictionPolicyError>;
    fn evict(&mut self) -> Result<Option<K>, EvictionPolicyError>;
}

#[derive(Debug, Clone)]
pub enum EvictionPolicyError {
    NotFound,
    InternalError(String),
}
impl std::fmt::Display for EvictionPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionPolicyError::NotFound => write!(f, "Item not found"),
            EvictionPolicyError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub struct LRUBasedEviction<K>
where
    K: Clone + Eq + std::hash::Hash,
{
    list: LinkedList<K>,
    // track: HashMap<K,>
}

impl<K> LRUBasedEviction<K>
where
    K: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }
}

impl<K> EvictionPolicy<K> for LRUBasedEviction<K>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
{
    fn process(&mut self, key: &K) -> Result<(), EvictionPolicyError> {
        println!("Processing key: {:?}", key);
        if self.list.contains(key) {
            // Move the key to the front of the list

            let mut filtered_list = self
                .list
                .clone()
                .into_iter()
                .filter(|x| x != key)
                .collect::<LinkedList<K>>();

            filtered_list.push_front(key.clone());
            self.list = filtered_list;
        } else {
            self.list.push_front(key.clone());
        }
        Ok(())
    }

    fn evict(&mut self) -> Result<Option<K>, EvictionPolicyError> {
        if let Some(key) = self.list.pop_back() {
            println!("Evicting key: {:?}", key);
            Ok(Some(key))
        } else {
            Err(EvictionPolicyError::NotFound)
        }
    }
}
