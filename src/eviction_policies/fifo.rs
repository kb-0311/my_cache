use std::collections::VecDeque;
use super::EvictionPolicy;

/// FIFO (First-In-First-Out) eviction policy based on a queue.
pub struct FifoPolicy<K> {
    queue: VecDeque<K>, // Queue to maintain the order of keys
}

impl<K> FifoPolicy<K> {
    /// Creates a new FIFO eviction policy.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl<K: Clone + PartialEq> EvictionPolicy<K> for FifoPolicy<K> {
    
    fn on_get(&mut self, _key: &K) {}

    /// Notifies the policy about a put operation with a new key.
    fn on_put(&mut self, key: K) {
        self.queue.push_back(key); // Adds the key to the back of the queue
    }

    /// Notifies the policy about a remove operation on a key.
    fn on_remove(&mut self, key: &K) {
        // Finds and removes the key from the queue
        if let Some(pos) = self.queue.iter().position(|x| x == key) {
            self.queue.remove(pos);
        }
    }

    /// Evicts and returns the oldest key according to FIFO order.
    fn evict(&mut self) -> Option<K> {
        self.queue.pop_front() // Removes and returns the frontmost key in the queue
    }
}
