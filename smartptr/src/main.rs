use crate::List::{Cons, Nil};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let c = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let _d = Cons(4, Box::new(c));

    // // This can cause compile error.
    // let _e = Cons(5, Box::new(c));

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

    let c = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("ref count of c = {}", Rc::strong_count(&c));
    {
        let _d = Cons(4, Rc::clone(&c));
        println!(
            "ref count of c after creating _d = {}",
            Rc::strong_count(&c)
        );
        {
            let _e = Cons(5, Rc::clone(&c));
            println!(
                "ref count of c after creating _e = {}",
                Rc::strong_count(&c)
            );
        }
        println!(
            "ref count of c after _e is out of scope = {}",
            Rc::strong_count(&c)
        );
    }
    println!(
        "ref count of c after _d is out of scope = {}",
        Rc::strong_count(&c)
    );
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

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
