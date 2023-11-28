use std::thread;
type JoinHandle = thread::JoinHandle<()>;

pub struct ThreadPool {
    threads: Vec<JoinHandle>,
};

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create some threads and store them in the vector
        }
        ThreadPool { threads }
    }

    // try to write a function named build with the following signature to compare with the new function:
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> 

    pub fn execute<F>(&self, f: F )
    where 
        F: FnOnce() + Send + 'static, 
    {
        //
    }
}