pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger.send("You're over your quota");
        } else if percentage >= 0.9 {
            self.messenger.send("You've used 90% of your quota!");
        } else if percentage >= 0.75 {
            self.messenger.send("You've used 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn over_75_triggers_message() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);
        tracker.set_value(80);
        assert_eq!(mock.messages.borrow().len(), 1);
    }

}
