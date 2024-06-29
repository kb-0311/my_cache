use my_cache::cache::Cache;
use my_cache::eviction_policies::{fifo::FifoPolicy, lru::LruPolicy, lifo::LifoPolicy, custom::RandomPolicy};

pub fn run_example() {
    let mut fifo_cache = Cache::new(2, FifoPolicy::new());
    fifo_cache.put(1, "one");
    fifo_cache.put(2, "two");
    println!("FIFO Cache: get(&1) = {:?}", fifo_cache.get(&1));
    fifo_cache.put(3, "three");
    println!("FIFO Cache after putting key 3: get(&1) = {:?}", fifo_cache.get(&1));
    println!("FIFO Cache after putting key 3: get(&2) = {:?}", fifo_cache.get(&2));
    println!("FIFO Cache after putting key 3: get(&3) = {:?}", fifo_cache.get(&3));

    let mut lru_cache = Cache::new(2, LruPolicy::new());
    lru_cache.put(1, "one");
    lru_cache.put(2, "two");

    println!("LRU Cache: get(&1) = {:?} , 1 becomes the most recently used key", lru_cache.get(&1));
    lru_cache.put(3, "three");
    println!("LRU Cache after putting key 3: get(&1) = {:?}", lru_cache.get(&1));
    println!("LRU Cache after putting key 3: get(&2) = {:?}", lru_cache.get(&2));
    println!("LRU Cache after putting key 3: get(&3) = {:?}", lru_cache.get(&3));

    let mut lifo_cache = Cache::new(2, LifoPolicy::new());
    lifo_cache.put(1, "one");
    lifo_cache.put(2, "two");
    println!("LIFO Cache: get(&1) = {:?}", lifo_cache.get(&1));
    lifo_cache.put(3, "three");
    println!("LIFO Cache after putting key 3: get(&1) = {:?}", lifo_cache.get(&1));
    println!("LIFO Cache after putting key 3: get(&2) = {:?}", lifo_cache.get(&2));
    println!("LIFO Cache after putting key 3: get(&3) = {:?}", lifo_cache.get(&3));

    let mut random_cache = Cache::new(3, RandomPolicy::new());
    random_cache.put(1, "one");
    random_cache.put(2, "two");
    random_cache.put(3, "three");
    println!("Random Cache: get(&1) = {:?}", random_cache.get(&1));
    println!("Random Cache: get(&2) = {:?}", random_cache.get(&2));
    println!("Random Cache: get(&3) = {:?}", random_cache.get(&3));

    random_cache.put(4, "four");
    println!("Random Cache after putting key 4: get(&1) = {:?}", random_cache.get(&1));
    println!("Random Cache after putting key 4: get(&2) = {:?}", random_cache.get(&2));
    println!("Random Cache after putting key 4: get(&3) = {:?}", random_cache.get(&3));
    println!("Random Cache after putting key 4: get(&4) = {:?}", random_cache.get(&4));

    let removed_value = random_cache.remove(&4).unwrap();
    println!("Removed value for key 4: {}", removed_value);
}
