use super::EvictionPolicy;
use rand::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub struct RandomPolicy<K> {
    map: HashMap<K, ()>,
    rng: ThreadRng,
}

impl<K: Hash + Eq + Clone> RandomPolicy<K> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            rng: thread_rng(),
        }
    }
}

impl<K: Hash + Eq + Clone> EvictionPolicy<K> for RandomPolicy<K> {
    fn on_get(&mut self, _key: &K) {}

    fn on_put(&mut self, key: K) {
        self.map.insert(key, ());
    }

    fn on_remove(&mut self, key: &K) {
        self.map.remove(key);
    }

    fn evict(&mut self) -> Option<K> {
        let keys: Vec<K> = self.map.keys().cloned().collect(); // Cloning keys to avoid lifetime issues

        if !keys.is_empty() {
            let index = self.rng.gen_range(0..keys.len());
            let key_to_evict = keys[index].clone(); // Clone the key to return

            self.map.remove(&key_to_evict);
            return Some(key_to_evict);
        }

        None
    }
}
