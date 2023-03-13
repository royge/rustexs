use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Error: Quota reached!");
        } else if percentage >= 0.9 {
            self.messenger.send("Warning: Qouta almost reached!");
        } else if percentage >= 0.75 {
            self.messenger.send("Warning: 75% quota reached!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
            // let mut borrow_one = self.sent_messages.borrow_mut();
            // let mut borrow_two = self.sent_messages.borrow_mut();

            // borrow_one.push(String::from(msg));
            // borrow_two.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let msgr = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&msgr, 100);

        limit_tracker.set_value(80);

        assert_eq!(msgr.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}
