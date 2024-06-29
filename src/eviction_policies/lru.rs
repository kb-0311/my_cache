use std::collections::HashMap;
use std::hash::Hash;
use super::EvictionPolicy;

pub struct LruPolicy<K> {
    map: HashMap<K, usize>,
    counter: usize,
}

impl<K: Eq + Hash + Clone + PartialEq> LruPolicy<K> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            counter: 0,
        }
    }
}

impl<K: Eq + Hash + Clone + PartialEq> EvictionPolicy<K> for LruPolicy<K> {
    fn on_get(&mut self, key: &K) {
        self.counter += 1;
        self.map.insert(key.clone(), self.counter);
    }

    fn on_put(&mut self, key: K) {
        self.counter += 1;
        self.map.insert(key, self.counter);
    }

    fn on_remove(&mut self, key: &K) {
        self.map.remove(key);
    }

    fn evict(&mut self) -> Option<K> {
        // Find the key to evict
        let key_to_evict = self.map.iter().min_by_key(|entry| entry.1).map(|(key, _)| key.clone());
        
        // Remove the key from the map if it exists
        if let Some(key) = key_to_evict {
            self.map.remove(&key);
            return Some(key);
        }
        None
    }
}
