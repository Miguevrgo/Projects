use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// A thread pool used to execute functions in parallel
///
/// Spawns a specified number of worker threads
///
/// # Examples
///
/// ```
/// let n_workers = 4;
/// let jobs: Vec<_> = //...//;
/// let pool = ThreadPool::new(n_workers)
///
/// for job in jobs {
///     pool.execute(move || {
///         complete_job(job);
///     })
/// }
///
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The argument represents the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if called with 0 threads
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(num_threads);

        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes provided function in parallel by sending it to a worker thread
    ///
    /// The argument is a closure that captures the environment
    ///
    pub fn execute<F>(&self, f: F)
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
            println!("Worder {} shutting down", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //TODO: std::thread::builder
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
