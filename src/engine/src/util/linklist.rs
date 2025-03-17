use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::marker::PhantomData;
use crate::util::linkable::Linkable;
pub struct LinkList<T> {
    sentinel: Rc<RefCell<Linkable>>,
    cursor: Option<Rc<RefCell<Linkable>>>,
    _phanton: PhantomData<T>,
}

impl<T> LinkList<T>
where
    T: AsRef<Rc<RefCell<Linkable>>> + From<Rc<RefCell<Linkable>>> 
{
    pub fn new() -> Self {
        let sentinel = Rc::new(RefCell::new(Linkable::new()));

        {
            let mut sentinel_mut = sentinel.borrow_mut();
            sentinel_mut.next = Some(Rc::clone(&sentinel));
            sentinel_mut.prev = Some(Rc::downgrade(&sentinel));
        }
        
        LinkList {
            sentinel,
            cursor: None,
            _phanton: PhantomData,
        }
    }

    pub fn add_tail(&mut self, node: Rc<RefCell<T>>) {
        let node_linkable = node.as_ref().borrow().as_ref().clone();

        {
            let node_borrowed = node_linkable.borrow();
            if node_borrowed.prev.is_some() {
                drop(node_borrowed);
                node_linkable.borrow_mut().unlink();
            }
        }

        let prev_weak = {
            let sentinel_borrowed = self.sentinel.borrow();
            sentinel_borrowed.prev.clone().unwrap()
        };

        let prev = prev_weak.upgrade().unwrap();

        {
            let mut node_mut = node_linkable.borrow_mut();
            node_mut.prev = Some(Rc::downgrade(&prev));
            node_mut.next = Some(Rc::clone(&self.sentinel));
        }

        {
            let mut prev_mut = prev.borrow_mut();
            prev_mut.next = Some(Rc::clone(&node_linkable));
        }

        {
            let mut sentinel_mut = self.sentinel.borrow_mut();
            sentinel_mut.prev = Some(Rc::downgrade(&node_linkable));
        }
    }

    pub fn add_head(&mut self, node: Rc<RefCell<T>>) {
        let node_linkable = node.as_ref().borrow().as_ref().clone();

        {
            let node_borrowed = node_linkable.borrow();
            if node_borrowed.prev.is_some() {
                drop(node_borrowed);
                node_linkable.borrow_mut().unlink();
            }
        }

        let next = {
            let sentinel_borrowed = self.sentinel.borrow();
            sentinel_borrowed.next.clone().unwrap()
        };

        {
            let mut node_mut = node_linkable.borrow_mut();
            node_mut.prev = Some(Rc::downgrade(&self.sentinel));
            node_mut.next = Some(next.clone());
        }

        {
            let mut sentinel_mut = self.sentinel.borrow_mut();
            sentinel_mut.next = Some(Rc::clone(&node_linkable));
        }

        {
            let mut next_mut = next.borrow_mut();
            next_mut.prev = Some(Rc::downgrade(&node_linkable));
        }
    }

    pub fn remove_head(&mut self) -> Option<Rc<RefCell<T>>> {
        let next = {
            let sentinel_borrowed = self.sentinel.borrow();
            sentinel_borrowed.next.clone().unwrap()
        };

        if Rc::ptr_eq(&next, &self.sentinel) {
            return None;
        }

        {
            next.borrow_mut().unlink();
        }

        let t_value = T::from(next);
        Some(Rc::new(RefCell::new(t_value)))
    }

    pub fn head(&mut self) -> Option<Rc<RefCell<T>>> {
        let next = {
            let sentinel_borrowed = self.sentinel.borrow();
            sentinel_borrowed.next.clone().unwrap()
        };

        if Rc::ptr_eq(&next, &self.sentinel) {
            self.cursor = None;
            return None;
        }

        {
            let next_borrowed = next.borrow();
            self.cursor = next_borrowed.next.clone();
        }

        let t_value = T::from(next);
        Some(Rc::new(RefCell::new(t_value)))
    }

    pub fn tail(&mut self) -> Option<Rc<RefCell<T>>> {
        let prev_weak = {
            let sentinel_borrowed = self.sentinel.borrow();
            sentinel_borrowed.prev.clone().unwrap()
        };

        let prev = prev_weak.upgrade().unwrap();

        if Rc::ptr_eq(&prev, &self.sentinel) {
            self.cursor = None;
            return None;
        }

        {
            let prev_borrowed = prev.borrow();
            self.cursor = prev_borrowed.prev.as_ref().and_then(|w| w.upgrade());
        }

        let t_value = T::from(prev);
        Some(Rc::new(RefCell::new(t_value)))
    }

    pub fn next(&mut self) -> Option<Rc<RefCell<T>>> {
        let cursor = self.cursor.take();

        if let Some(node) = cursor {
            if Rc::ptr_eq(&node, &self.sentinel) {
                return None;
            }

            let result = Rc::clone(&node);

            {
                let node_borrowed = node.borrow();
                if let Some(next) = &node_borrowed.next {
                    self.cursor = Some(Rc::clone(next));
                }
            }

            let t_value = T::from(result);
            return Some(Rc::new(RefCell::new(t_value)));
        }

        None
    }

    pub fn prev(&mut self) -> Option<Rc<RefCell<T>>> {
        let cursor = self.cursor.take();

        if let Some(node) = cursor {
            if Rc::ptr_eq(&node, &self.sentinel) {
                return None;
            }

            let result = Rc::clone(&node);

            {
                let node_borrowed = node.borrow();
                if let Some(prev_weak) = &node_borrowed.prev {
                    if let Some(prev) = prev_weak.upgrade() {
                        self.cursor = Some(prev);
                    }
                }
            }

            let t_value = T::from(result);
            return Some(Rc::new(RefCell::new(t_value)));
        }

        None
    }

    pub fn clear(&mut self) {
        loop {
            let next = {
                let sentinel_borrowed = self.sentinel.borrow();
                sentinel_borrowed.next.clone().unwrap()
            };

            if Rc::ptr_eq(&next, &self.sentinel) {
                return;
            }

            next.borrow_mut().unlink();
        }
    }
}