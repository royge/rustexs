use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    simple();
    moved();
    msgpassing();
}

fn simple() {
    let handle = thread::spawn(|| {
        for i in 1..30 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..13 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn moved() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("vector: {:?}", v));

    // drop(v);

    handle.join().unwrap();
}

fn msgpassing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val: {:?}", val)
    });

    let received = rx.recv().unwrap();
    println!("got: {:?}", received);

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hey"),
            String::from("hola"),
            String::from("thread"),
            String::from("passing"),
        ];

        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(900));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("Rust"),
            String::from("message"),
        ];

        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for v in rx {
        println!("{:?}", v);
    }
}
