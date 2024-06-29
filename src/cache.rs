use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use crate::eviction_policies::EvictionPolicy;

pub struct Cache<K, V, P>
where
    K: Eq + Hash + Clone,
    P: EvictionPolicy<K>,
{
    store: HashMap<K, V>,
    policy: P,
    capacity: usize,
    lock: Arc<Mutex<()>>,
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
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let _lock = self.lock.lock().unwrap();
        self.policy.on_get(key);
        self.store.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        let _lock = self.lock.lock().unwrap();
        if self.store.len() >= self.capacity {
            if let Some(evicted_key) = self.policy.evict() {
                self.store.remove(&evicted_key);
            }
        }
        self.policy.on_put(key.clone());
        self.store.insert(key, value);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let _lock = self.lock.lock().unwrap();
        self.policy.on_remove(key);
        self.store.remove(key)
    }
}
