pub mod fifo;
pub mod lru;
pub mod lifo;
pub mod custom;

pub trait EvictionPolicy<K> {
    fn on_get(&mut self, key: &K);
    fn on_put(&mut self, key: K);
    fn on_remove(&mut self, key: &K);
    fn evict(&mut self) -> Option<K>;
}
