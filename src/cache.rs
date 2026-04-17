use std::hash::Hash;
use std::collections::HashMap;
use std::time::{ Instant, Duration };

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    ttl: Duration,
}

impl<V> CacheEntry<V> {
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

#[derive(Debug)]
pub struct Cache<K, V> {
    storage: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K, V> Cache<K, V> where K: Eq + Hash {
    pub fn new(default_seconds: u64) -> Self {
        Cache {
            storage: HashMap::new(),
            default_ttl: Duration::from_secs(default_seconds),
        }
    }

    pub fn set(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let expiration = ttl.unwrap_or(self.default_ttl);

        let entry = CacheEntry {
            value: value,
            created_at: Instant::now(),
            ttl: expiration,
        };

        self.storage.insert(key, entry);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let is_expired = if let Some(entry) = self.storage.get(key) {
            entry.is_expired()
        } else {
            return None;
        };

        if is_expired {
            self.storage.remove(&key);
            None
        } else {
            self.storage.get(&key).map(|entry| &entry.value)
        }
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key).map(|entry| entry.value)
    }

    pub fn exists(&mut self, key: &K) -> bool {
        let expired = match self.storage.get(key) {
            Some(entry) => entry.is_expired(),
            None => {
                return false;
            }
        };

        if expired {
            self.storage.remove(key);
            return false;
        } else {
            return true;
        }
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }
    pub fn clear(&mut self) {
        self.storage.clear()
    }

    pub fn cleanup(&mut self)->usize{
        let original_size = self.storage.len();

        self.storage.retain(|_, entry| !entry.is_expired());

        let new_size = self.storage.len();

        original_size-new_size
    }
}