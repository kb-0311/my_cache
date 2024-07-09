use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use my_cache::cache_without_internal_lock::Cache;
use my_cache::eviction_policies::lru::LruPolicy;

/// Example function to demonstrate concurrent modifications without internal lock
pub fn concurrent_operations_demo() {
    // Create a thread-safe LRU cache without internal lock
    let cache: Arc<Mutex<Cache<i32, String, _>>> = Arc::new(Mutex::new(Cache::new(5, LruPolicy::new())));

    // Clone Arc for sharing across threads
    let cache_clone: Arc<Mutex<Cache<i32, String, _>>> = Arc::clone(&cache);

    // Spawn threads to perform concurrent cache operations
    let thread_handles: Vec<_> = (0..10).map(|i| {
        let cache_clone = Arc::clone(&cache_clone);
        thread::spawn(move || {
            let mut cache = cache_clone.lock().unwrap();
            cache.put(i, format!("value {}", i));

            // println!("Thread {} added value {}", i, i);

            let value = cache.get(&i);
            // let value1 = cache.get(&(i-1));


            println!("Thread {} retrieved value {:?}", i, value);


            // println!("Thread {} retrieved value {:?}", i, value1);

        })
    }).collect();

    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().unwrap();
    }
}