use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::util::doubly_linkable::DoublyLinkable;

pub struct DoublyLinkList {
    pub head: Rc<RefCell<DoublyLinkable>>,
}

impl DoublyLinkList {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(DoublyLinkable::new()));

        {
            let mut head_mut = head.borrow_mut();
            head_mut.next2 = Some(Rc::clone(&head));
            head_mut.prev2 = Some(Rc::downgrade(&head));
        }

        DoublyLinkList { head }
    }

    pub fn push(&mut self, node: Rc<RefCell<DoublyLinkable>>) {
        {
            let node_borrowed = node.borrow();
            if node_borrowed.prev2.is_some() {
                drop(node_borrowed);
                node.borrow_mut().unlink2();
            }
        }

        let prev2_weak = {
            let head_borrowed = self.head.borrow();
            head_borrowed.prev2.clone().unwrap()
        };

        let prev2 = prev2_weak.upgrade().unwrap();

        {
            let mut node_mut = node.borrow_mut();
            node_mut.prev2 = Some(Rc::downgrade(&prev2));
            node_mut.next2 = Some(Rc::clone(&self.head));
        }

        {
            let mut prev2_mut = prev2.borrow_mut();
            prev2_mut.next2 = Some(Rc::clone(&node));
        }

        {
            let mut head_mut = self.head.borrow_mut();
            head_mut.prev2 = Some(Rc::downgrade(&node));
        }
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<DoublyLinkable>>> {
        let next2 = {
            let head_borrowed = self.head.borrow();
            head_borrowed.next2.clone()
        };

        return if Rc::ptr_eq(&next2.as_ref().unwrap(), &self.head) {
            None
        } else {
            {
                let mut node_mut = next2.as_ref().unwrap().borrow_mut();
                node_mut.unlink2();
            }
            next2
        }
    }
}