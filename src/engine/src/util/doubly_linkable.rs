use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct DoublyLinkable {
    // "Inheriting" from [Linkable]
    pub key: u64,
    pub next: Option<Rc<RefCell<DoublyLinkable>>>,
    pub prev: Option<Weak<RefCell<DoublyLinkable>>>,

    // DoublyLinkable specific fields
    pub next2: Option<Rc<RefCell<DoublyLinkable>>>,
    pub prev2: Option<Weak<RefCell<DoublyLinkable>>>,
}

impl DoublyLinkable {
    pub fn new() -> Self {
        DoublyLinkable {
            key: 0,
            next: None,
            prev: None,
            next2: None,
            prev2: None,
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

    pub fn unlink2(&mut self) {
        if let Some(prev2) = self.prev2.take() {
            if let Some(prev2_strong) = prev2.upgrade() {
                let mut prev2_borrowed = prev2_strong.borrow_mut();
                prev2_borrowed.next2 = self.next2.take();

                if let Some(next2_ref) = &prev2_borrowed.next2 {
                    let mut next2_borrowed = next2_ref.borrow_mut();
                    next2_borrowed.prev2 = Some(Weak::clone(&Rc::downgrade(&prev2_strong)));
                }
            }
        }
    }
}