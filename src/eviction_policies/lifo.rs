use super::EvictionPolicy;

pub struct LifoPolicy<K> {
    stack: Vec<K>,
}

impl<K> LifoPolicy<K> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
}

impl<K: Clone + PartialEq> EvictionPolicy<K> for LifoPolicy<K> {
    fn on_get(&mut self, _key: &K) {}

    fn on_put(&mut self, key: K) {
        self.stack.push(key);
    }

    fn on_remove(&mut self, key: &K) {
        if let Some(pos) = self.stack.iter().position(|x| x == key) {
            self.stack.remove(pos);
        }
    }

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
