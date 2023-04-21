use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs;
use std::io::prelude::*;
use std::io::{Write};
use std::time::Duration;

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

pub fn run_server(
    addr: &str,
    running: Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;

    if let Err(err) = listener.set_nonblocking(true) {
        return Err(Box::new(err));
    }

    let pool = ThreadPool::new(8);

    let thread = thread::spawn(move || {
        match pool {
            Ok(pool) => loop {
                match listener.accept() {
                    Ok((stream, _)) => {
                        pool.execute(move || handle_connection(stream));
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if !running.load(Ordering::SeqCst) {
                            break;
                        }
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            },
            Err(err) => {
                println!("Unable to initial pool properly, {err}");
            }
        };
    });

    if let Err(_) = thread.join() {
        return Err("Unable to join thread".into());
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(_) => {
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Do nothing, continue looping
                continue;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "wakeup.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
