use super::EvictionPolicy;

/// LIFO (Last-In-First-Out) eviction policy based on a stack.
pub struct LifoPolicy<K> {
    stack: Vec<K>, // Stack to maintain the order of keys
}

impl<K> LifoPolicy<K> {
    /// Creates a new LIFO eviction policy.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
}

impl<K: Clone + PartialEq> EvictionPolicy<K> for LifoPolicy<K> {
    /// Notifies the policy about a get operation on a key.
    fn on_get(&mut self, _key: &K) {}

    /// Notifies the policy about a put operation with a new key.
    fn on_put(&mut self, key: K) {
        self.stack.push(key); // Pushes the key onto the stack
    }

    /// Notifies the policy about a remove operation on a key.
    fn on_remove(&mut self, key: &K) {
        // Finds and removes the key from the stack
        if let Some(pos) = self.stack.iter().position(|x| x == key) {
            self.stack.remove(pos);
        }
    }

    /// Evicts and returns the most recently inserted key (top of stack).
    fn evict(&mut self) -> Option<K> {
        // Find the key to evict (last inserted key)
        let key_to_evict = self.stack.last().cloned();
        
        // Remove the key from the stack if it exists
        if let Some(key) = &key_to_evict {
            self.stack.retain(|x| x != key);
            return key_to_evict;
        }
        None
    }
}
