pub mod errors;
mod jobs;
mod workers;

use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use errors::PoolCreationError;
use jobs::Job;
use workers::Worker;

pub struct ThreadPool<T> {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker<T>>,
}

impl ThreadPool<()> {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool. The number of threads
    /// must be greater than zero. Otherwise, this function will error with
    /// PoolCreationError.
    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        (size > 0)
            .then_some(ThreadPool::<()>::init(size))
            .ok_or(PoolCreationError::new(
                "number of threads must be greater than zero",
            ))
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .expect(" Should have a sender")
            .send(job)
            .unwrap();
    }

    fn init(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = {
            let mut vec = Vec::with_capacity(size);
            vec.extend((0..size).map(|id| Worker::new(id, Arc::clone(&receiver))));
            vec
        };
        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }
}

impl<T> Drop for ThreadPool<T> {
    fn drop(&mut self) {
        drop(self.sender.take());
        self.workers.iter_mut().for_each(|worker| {
            println!("Shutting down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        });
    }
}
