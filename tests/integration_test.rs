use my_cache::cache::Cache;
use my_cache::eviction_policies::{fifo::FifoPolicy, lru::LruPolicy, lifo::LifoPolicy , custom::RandomPolicy};

#[test]
fn test_fifo_cache() {
    let mut fifo_cache = Cache::new(2, FifoPolicy::new());
    fifo_cache.put(1, "one");
    fifo_cache.put(2, "two");
    assert_eq!(fifo_cache.get(&1), Some(&"one"));
    fifo_cache.put(3, "three");
    assert_eq!(fifo_cache.get(&1), None); // 1 should be evicted
    assert_eq!(fifo_cache.get(&2), Some(&"two"));
    assert_eq!(fifo_cache.get(&3), Some(&"three"));
}

#[test]
fn test_lru_cache() {
    // Initialize a new LRU cache with capacity 2
    let mut lru_cache = Cache::new(2, LruPolicy::new());

    // Insert two entries into the cache
    lru_cache.put(1, "one");
    lru_cache.put(2, "two");

    // Verify initial entries are in cache
    assert_eq!(lru_cache.get(&1), Some(&"one"));
    assert_eq!(lru_cache.get(&2), Some(&"two"));

    // Insert a third entry to trigger eviction
    lru_cache.put(3, "three");

    // Ensure correct eviction and remaining entries
    assert_eq!(lru_cache.get(&1) , None);    // 1 should still be in cache
    assert_eq!(lru_cache.get(&2),  Some(&"two"));            // 2 should be evicted
    assert_eq!(lru_cache.get(&3), Some(&"three"));  // 3 should be in cache now
}


#[test]
fn test_lifo_cache() {
    let mut lifo_cache = Cache::new(2, LifoPolicy::new());
    lifo_cache.put(1, "one");
    lifo_cache.put(2, "two");
    assert_eq!(lifo_cache.get(&1), Some(&"one"));
    lifo_cache.put(3, "three");
    assert_eq!(lifo_cache.get(&1), Some(&"one"));
    assert_eq!(lifo_cache.get(&2), None); // 2 should be evicted
    assert_eq!(lifo_cache.get(&3), Some(&"three"));
}


#[test]
fn test_random_cache() {
    let mut random_cache = Cache::new(3, RandomPolicy::new());
    random_cache.put(1, "one");
    random_cache.put(2, "two");
    random_cache.put(3, "three");

    // Ensure all items are in the cache
    assert_eq!(random_cache.get(&1), Some(&"one"));
    assert_eq!(random_cache.get(&2), Some(&"two"));
    assert_eq!(random_cache.get(&3), Some(&"three"));

    // Insert one more item to trigger eviction
    random_cache.put(4, "four");

    // Determine which key was evicted
    let evicted_key = if random_cache.get(&1).is_none() {
        1
    } else if random_cache.get(&2).is_none() {
        2
    } else if random_cache.get(&3).is_none() {
        3
    } else {
        panic!("Expected one of the keys (1, 2, 3) to be evicted");
    };

    // Ensure the remaining items are still in cache
    assert_eq!(random_cache.get(&1).is_some(), evicted_key != 1);
    assert_eq!(random_cache.get(&2).is_some(), evicted_key != 2);
    assert_eq!(random_cache.get(&3).is_some(), evicted_key != 3);
    assert_eq!(random_cache.get(&4), Some(&"four"));
}