use ctrlc;
use hello::{run_server};
use std::io::{self};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(err) = run_server("127.0.0.1:7878", running) {
        panic!("Can't start server because of this error, {err}")
    }

    Ok(())
}
