use std::sync::{Arc, Mutex};
// use std::thread;
use std::collections::HashMap;
use std::hash::Hash;
use crate::eviction_policies::EvictionPolicy;

/// Example cache struct without an internal lock
pub struct Cache<K, V, P>
where
    K: Eq + Hash + Clone,
    P: EvictionPolicy<K>,
{
    store: HashMap<K, V>,
    policy: P,
    capacity: usize,
    // Arc and Mutex for thread-safe operations
    // lock: Arc<Mutex<()>>,
}

impl<K, V, P> Cache<K, V, P>
where
    K: Eq + Hash + Clone,
    P: EvictionPolicy<K>,
{
    pub fn new(capacity: usize, policy: P) -> Self {
        Self {
            store: HashMap::new(),
            policy: policy,
            capacity: capacity,
            // lock: Arc::new(Mutex::new(())),
        }
    }

    /// Retrieves a value from the cache based on the provided key.
    /// Returns `Some(&V)` if the key exists, otherwise `None`.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.policy.on_get(key);
        self.store.get(key)
    }

    /// Inserts a key-value pair into the cache.
    /// If the cache exceeds its capacity, it may trigger eviction based on the policy.
    pub fn put(&mut self, key: K, value: V) {
        if self.store.len() >= self.capacity {
            if let Some(evicted_key) = self.policy.evict() {
                self.store.remove(&evicted_key);
            }
        }
        self.policy.on_put(key.clone());
        self.store.insert(key, value);
    }

    /// Removes a key-value pair from the cache based on the provided key.
    /// Returns `Some(V)` of the removed value if the key existed, otherwise `None`.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.policy.on_remove(key);
        self.store.remove(key)
    }
}
