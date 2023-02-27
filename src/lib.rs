use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job. Executing.");
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// n is the number of threads in the pool
    ///
    /// Panics if n is zero
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(n);
        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn run<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}
