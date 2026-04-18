// use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::collections::HashMap;
use std::time::{ Instant, Duration };

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    ttl: Duration,
    last_accessed: Instant
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
    capacity: usize
}

impl<K, V> Cache<K, V> where K: Eq + Hash + Clone {
    pub fn new(default_seconds: u64, capacity: usize) -> Self {
        Cache {
            storage: HashMap::new(),
            default_ttl: Duration::from_secs(default_seconds),
            capacity
        }
    }

    pub fn set(&mut self, key: K, value: V, ttl: Option<Duration>) {
        if self.storage.len() >= self.capacity && !self.storage.contains_key(&key){
            self.evict_oldest();
        }

        let expiration = ttl.unwrap_or(self.default_ttl);

        let entry = CacheEntry {
            value: value,
            created_at: Instant::now(),
            ttl: expiration,
            last_accessed: Instant::now()
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
            let entry = self.storage.get_mut(key).unwrap();
            entry.last_accessed = Instant::now();
            Some(&entry.value)
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

    fn evict_oldest(&mut self){
        let mut oldest_key: Option<K> = None;
        let mut oldest_time = Instant::now();

        for (key, entry) in self.storage.iter(){
            if entry.last_accessed < oldest_time{
                oldest_time = entry.last_accessed;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key{
            self.storage.remove(&key);
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