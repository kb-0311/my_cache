use std::collections::VecDeque;
use std::hash::Hash;
use super::EvictionPolicy;

/// LRU (Least Recently Used) eviction policy based on a deque.
pub struct LruPolicy<K> {
    deque: VecDeque<K>, // Deque to maintain the access order of keys
}

impl<K> LruPolicy<K> {
    /// Creates a new LRU eviction policy.
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }
}

impl<K: Clone + PartialEq + Hash> EvictionPolicy<K> for LruPolicy<K> {
    /// Notifies the policy about a get operation with a key.
    fn on_get(&mut self, key: &K) {
        // When a key is accessed, move it to the back of the deque
        self.deque.retain(|k| k != key); // Ensure only one copy of the key exists
        self.deque.push_back(key.clone());
    }

    /// Notifies the policy about a put operation with a new key.
    fn on_put(&mut self, key: K) {
        // Add the new key to the back of the deque
        self.deque.push_back(key);
    }

    /// Notifies the policy about a remove operation on a key.
    fn on_remove(&mut self, key: &K) {
        // Remove the key from the deque if it exists
        self.deque.retain(|k| k != key);
    }

    /// Evicts and returns the least recently used key according to LRU order.
    fn evict(&mut self) -> Option<K> {
        self.deque.pop_front() // Remove and return the frontmost key in the deque
    }
}
