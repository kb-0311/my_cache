use std::collections::VecDeque;
use super::EvictionPolicy;

pub struct FifoPolicy<K> {
    queue: VecDeque<K>,
}

impl<K> FifoPolicy<K> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl<K: Clone + PartialEq> EvictionPolicy<K> for FifoPolicy<K> {
    fn on_get(&mut self, _key: &K) {}

    fn on_put(&mut self, key: K) {
        self.queue.push_back(key);
    }

    fn on_remove(&mut self, key: &K) {
        if let Some(pos) = self.queue.iter().position(|x| x == key) {
            self.queue.remove(pos);
        }
    }

    fn evict(&mut self) -> Option<K> {
        self.queue.pop_front()
    }
}
