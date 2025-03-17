use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Linkable {
    pub key: u64,
    pub next: Option<Rc<RefCell<Linkable>>>,
    pub prev: Option<Weak<RefCell<Linkable>>>,
}

impl Linkable {
    pub fn new() -> Self {
        Linkable {
            key: 0,
            next: None,
            prev: None,
        }
    }

    pub fn unlink(&mut self) {
        if let Some(prev) = self.prev.take() {
            if let Some(prev_strong) = prev.upgrade() {
                let mut prev_borrowed = prev_strong.borrow_mut();
                prev_borrowed.next = self.next.take();

                if let Some(next_ref) = &prev_borrowed.next {
                    let mut next_borrowed = next_ref.borrow_mut();
                    next_borrowed.prev = Some(Weak::clone(&Rc::downgrade(&prev_strong)));
                }
            }
        }
    }
}