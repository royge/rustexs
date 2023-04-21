use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// The `new` function will return PoolCreationError when size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::new("Empty pool size.".into()));
        }

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers!");

        for worker in &mut self.workers {
            if let Err(err) = self.sender.send(Message::Terminate) {
                println!(
                    "Error sending termination message to worker {}; {}",
                    worker.id, err
                );
            }
        }

        println!("Shutting down all workers!");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PoolCreationError {
    message: String,
}

impl PoolCreationError {
    fn new(message: String) -> PoolCreationError {
        PoolCreationError { message }
    }
}

impl Error for PoolCreationError {}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    (job)();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thread_pool() {
        if let Ok(_) = ThreadPool::new(0) {
            panic!("Should return PoolCreationError.")
        }

        if let Err(_) = ThreadPool::new(2) {
            panic!("Should return new ThreadPool.")
        }

        let pool = ThreadPool::new(4).unwrap();
        assert_eq!(4, pool.workers.len());
    }

    #[test]
    fn test_new_pool_creation_error() {
        let err = PoolCreationError::new(String::from("Test error"));
        assert_eq!("Test error", format!("{}", err));
    }

    #[test]
    fn test_new_worker() {
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut worker = Worker::new(1, receiver);

        thread::spawn(move || {
            sender.send(Message::Terminate).unwrap();
        });

        let result = worker.thread.take().expect("Can't take").join();
        assert!(result.is_ok());
    }
}
