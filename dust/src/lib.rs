use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

type JoinHandle = Option<thread::JoinHandle<()>>;

pub struct ThreadPool {
    // threads: Vec<JoinHandle>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job =  Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: JoinHandle,
}

// Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {

        let thread = thread::spawn( move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

        });

        Worker { id, thread: Some(thread) }
    }
}

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
        let mut workers = Vec::with_capacity(size);
        // In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    // try to write a function named build with the following signature to compare with the new function:
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> 

    pub fn execute<F>(&mut self, f: F )
    where 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            drop(self.sender.take());
            // worker.thread.take().unwrap().join().unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}