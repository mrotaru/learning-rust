pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Over 75% of quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Over 90% of quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Quota consumed!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // this will result in a panic at runtime:
            // already borrowed: BorrowMutError
            // let mut mut_1 = self.sent_messages.borrow_mut();
            // let mut mut_2 = self.sent_messages.borrow_mut();
            // mut_1.push(String::from(message));
            // mut_2.push(String::from(message));

            // this will make the test fail, but not because of
            // a panic; it will fail because the count will be 2
            // instead of 1. I guess it does not panic because the
            // two mutable references are not "live" at the same time
            // self.sent_messages.borrow_mut().push(String::from(message));
            // self.sent_messages.borrow_mut().push(String::from(message));

            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message () {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main() {
    println!("Hello, world!");
}
