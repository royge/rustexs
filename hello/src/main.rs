use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::io::{self, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

extern crate ctrlc;

fn main() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

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
                println!("Unable to start server properly: {}", err);
            }
        };
    });

    thread.join().unwrap();

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(_) => { break; }
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
