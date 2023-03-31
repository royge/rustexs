use crate::rc::List::{Cons as RcCons, Nil as RcNil};
use crate::rccell::List::{Cons as RcCellCons, Nil as RcCellNil};
use crate::refcycle::List::{Cons as RefCyCons, Nil as RefCyNil};
use crate::List::{Cons, Nil};
use smartptr::Node;
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    let b = Box::new(5);
    assert_eq!(5, *b);

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
    assert_eq!(1, Rc::strong_count(&c));
    {
        let _d = RcCons(4, Rc::clone(&c));
        assert_eq!(2, Rc::strong_count(&c));

        {
            let _e = RcCons(5, Rc::clone(&c));
            assert_eq!(3, Rc::strong_count(&c));
        }
        assert_eq!(2, Rc::strong_count(&c));
    }
    assert_eq!(1, Rc::strong_count(&c));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcCellCons(Rc::clone(&value), Rc::new(RcCellNil)));

    let b = RcCellCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RcCellCons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    assert_eq!(15, a.value());
    assert_eq!(15, b.next().expect("valid next").value());
    assert_eq!(15, c.next().expect("valid next").value());

    let a = Rc::new(RefCyCons(5, RefCell::new(Rc::new(RefCyNil))));

    assert_eq!(1, Rc::strong_count(&a));
    assert_eq!(&RefCyNil, a.tail().unwrap().borrow().as_ref());

    let b = Rc::new(RefCyCons(10, RefCell::new(Rc::clone(&a))));

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(1, Rc::strong_count(&b));

    // println!("b next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b)
    }

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(2, Rc::strong_count(&b));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item: {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    assert_eq!(1, Rc::strong_count(&leaf));
    assert_eq!(0, Rc::weak_count(&leaf));

    let res = leaf.parent.borrow().upgrade();
    assert_eq!("None", format!("{:?}", res));

    assert_eq!(0, Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        assert_eq!(2, Rc::strong_count(&leaf));

        assert_eq!(1, Rc::strong_count(&branch));
        assert_eq!(0, Rc::weak_count(&branch));

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        assert_eq!(1, Rc::strong_count(&branch));
        assert_eq!(1, Rc::weak_count(&branch));

        assert_eq!(2, Rc::strong_count(&leaf));
        assert_eq!(0, Rc::weak_count(&leaf));

        let res = leaf.parent.borrow().upgrade();
        assert_ne!("None", format!("{:?}", res));
    }

    let res = leaf.parent.borrow().upgrade();
    assert_eq!("None", format!("{:?}", res));

    assert_eq!(1, Rc::strong_count(&leaf));
    assert_eq!(0, Rc::weak_count(&leaf));
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
    use crate::rccell::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    impl List {
        pub fn value(&self) -> i32 {
            match self {
                Cons(v, _) => *v.borrow(),
                Nil => 0,
            }
        }

        pub fn next(&self) -> Option<&Rc<List>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
}

mod refcycle {
    use crate::refcycle::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
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
