use hello;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use reqwest::blocking::Client;
use http;

#[test]
fn test_with_http_client() {
    let running = Arc::new(AtomicBool::new(true));

    let running_clone = running.clone();

    let thread = thread::spawn(|| {
        if let Err(err) = hello::run_server("127.0.0.1:7878", running) {
            panic!("Can't start server because of this error, {err}")
        }
    });

    let client = Client::new();

    let res = client.get("http://127.0.0.1:7878").send().unwrap();
    assert!(res.status().is_success());

    let res = client.get("http://127.0.0.1:7878/notfound").send().unwrap();
    assert_eq!(res.status().as_u16(), http::StatusCode::NOT_FOUND);

    // let res = client.get("http://127.0.0.1:7878/sleep").send().unwrap();
    // assert!(res.status().is_success());

    running_clone.store(false, Ordering::SeqCst);
    thread.join().unwrap();
}
