use std::sync::{Arc, Mutex};
use my_cache::cache::Cache;
use my_cache::eviction_policies::lru::LruPolicy;

pub fn run_threads() {
    // Create a thread-safe LRU cache
    let cache = Arc::new(Mutex::new(Cache::new(100, LruPolicy::new())));
    
    // Clone Arc for sharing across threads
    let cache_clone = Arc::clone(&cache);
    
    // Spawn threads to perform concurrent cache operations
    let thread_handles: Vec<_> = (0..10).map(|i| {
        let cache = Arc::clone(&cache_clone);
        std::thread::spawn(move || {
            let mut cache = cache.lock().unwrap();
            cache.put(i, format!("value {}", i));
            println!("Thread {} added value {}", i, i);
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().unwrap();
    }
}