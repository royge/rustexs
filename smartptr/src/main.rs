use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::fmt;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(6);

    assert_eq!(5, x);
    assert_eq!(6, *y);

    hello("rust");
    hello(&MyBox::new(String::from("Rust")));

    println!("Rust MyBox created");

    let b = MyBox::new(100);
    drop(b);
    println!("End of main");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}", name)
}

struct MyBox<T: fmt::Display>(T);

impl<T: fmt::Display> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T: fmt::Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox: {}", self.0);
    }
}
