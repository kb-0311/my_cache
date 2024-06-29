mod example {
    pub mod cache_example;
    pub mod multi_threading_example;
}

use example::cache_example::run_example;
use example::multi_threading_example::run_threads;

fn main() {
    run_example();
    run_threads();
}
