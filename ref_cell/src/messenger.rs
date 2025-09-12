use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    /// Create a new Tracker with the given max quota
    pub fn new(max: usize) -> Tracker {
        Tracker {
            messages: Rc::new(RefCell::new(Vec::new())),
            value: RefCell::new(0),
            max,
        }
    }

    /// Update the tracker’s value based on the given Rc
    pub fn set_value<T>(&self, val: &Rc<T>) {
        let count = Rc::strong_count(val);

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            *self.value.borrow_mut() = count;

            let percent = (count * 100) / self.max;
            if percent > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percent
                ));
            }
        }
    }

    /// Peek at the potential usage of the given Rc without updating value
    pub fn peek<T>(&self, val: &Rc<T>) {
        let count = Rc::strong_count(val);
        let percent = (count * 100) / self.max;

        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percent
        ));
    }
}
