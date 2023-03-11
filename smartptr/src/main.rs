use crate::List::{Cons, Nil};
use crate::rc::List::{Cons as RcCons, Nil as RcNil};
use crate::rccell::List::{Cons as RcCellCons, Nil as RcCellNil};
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let c = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let _d = Cons(4, Box::new(c));

    // This can cause compile error.
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

    let c = Rc::new(RcCons(
        1,
        Rc::new(RcCons(2, Rc::new(RcCons(3, Rc::new(RcNil))))),
    ));
    println!("ref count of c = {}", Rc::strong_count(&c));
    {
        let _d = RcCons(4, Rc::clone(&c));
        println!(
            "ref count of c after creating _d = {}",
            Rc::strong_count(&c)
        );
        {
            let _e = RcCons(5, Rc::clone(&c));
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

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcCellCons(Rc::clone(&value), Rc::new(RcCellNil)));

    let b = RcCellCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RcCellCons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

mod rc {
    use std::rc::Rc;

    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
}

mod rccell {
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
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
