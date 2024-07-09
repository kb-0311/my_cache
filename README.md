# In-Memory Caching Library

This Rust library provides an in-memory caching system with support for multiple standard eviction policies and the ability to add custom eviction policies.

## Features

The main features this library supports are :

1. Support for multiple Standard Eviction Policies ( FIFO, LRU, LIFO )
2. Support to add custom eviction policies.
3. Thread Safety

### Standard Eviction Policies

The library supports the following standard eviction policies:

- **FIFO (First-In-First-Out)**: Evicts the oldest inserted items first.
- **LRU (Least Recently Used)**: Evicts the least recently accessed items first.
- **LIFO (Last-In-First-Out)**: Evicts the most recently inserted items first.

### Custom Eviction Policies

Users can easily define and integrate custom eviction policies by implementing the `EvictionPolicy` trait.

### Thread Safety

The library ensures thread safety using `Arc<Mutex<()>>` to manage concurrent access to critical sections of code. Here's how it works:

#### Mutex (Mutual Exclusion)

- **Mutex** stands for mutual exclusion. It allows only one thread at a time to access shared data. In Rust, `Mutex` is a synchronization primitive provided by the standard library.
- When a thread acquires a lock on the `Mutex`, it gains exclusive access to the data protected by the `Mutex`. Other threads attempting to acquire the lock will block until it is released.

#### Arc (Atomic Reference Counting)

- **Arc** (Atomic Reference Counting) allows multiple threads to have shared ownership of the same data. It ensures that the underlying data is not destroyed until all references to it are gone.
- It is used here (`Arc<Mutex<()>>`) to share ownership of a `Mutex` across multiple threads. Each thread can clone the `Arc`, incrementing the reference count, and safely access the protected data.

#### Implementation

- In the caching library, each instance of `Cache` uses a shared `Arc<Mutex<()>>` (`lock`) to guard critical operations (`put`, `get`, `remove`) against concurrent access.
- When a thread wants to perform an operation on the cache, it first acquires a lock on the `Mutex` by calling `lock.lock().unwrap()`. This ensures that only one thread can execute a critical section at a time.
- Once the operation is completed, the lock is automatically released when the `MutexGuard` returned by `lock()` goes out of scope.

## How to use

Make sure you have rust and cargo installed on you system.

```bash
git clone https://github.com/kb-0311/my_cache.git
cd my_cache
cargo build
cargo run
```

This runs the `src/main.rs` file that executed two examples in the `src/examples/` directory. Feel free to edit the files to play around with the cache. Treat the examples directory as a playground.

Standard Eviction Policies are defined in the `src/eviction_policies`. There is a `custom.rs` file in that directory that makes it possible to define user defined eviction policy. I have just used a Random eviction as a place holder for now.

Last but not the Least, `src/cache.rs` contains the actual cache definition.
