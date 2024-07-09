mod example {
    pub mod cache_example;
    pub mod multi_threading_example;
    pub mod multi_threading_without_internal_lock;
}

use example::cache_example::run_example;
use example::multi_threading_example::run_threads;
use example::multi_threading_without_internal_lock::concurrent_operations_demo;


fn main() {
    run_example();
    run_threads();
    concurrent_operations_demo();
}
