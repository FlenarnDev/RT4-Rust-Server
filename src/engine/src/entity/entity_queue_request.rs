use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::script::script_file::ScriptFile;

pub enum NPCQueueType {
    Normal,
}

pub enum PlayerQueueType {
    Normal,
    Long, // Like normal, but with dev-controlled logout behavior.
    Engine,
    Weak, // Added September 2004
    Strong, // Added late 2004
    Soft, // Added in OSRS (Only then? Or by 2009?)
}

pub enum QueueType {
    NPC(NPCQueueType),
    Player(PlayerQueueType),
}

pub enum ScriptArgument {
    Number(i32),
    String(String),
}

pub struct EntityQueueRequest {
    pub key: u64,
    pub next: Option<Rc<RefCell<EntityQueueRequest>>>,
    pub prev: Option<Weak<RefCell<EntityQueueRequest>>>,
    
    pub queue_type: QueueType,
    pub script: ScriptFile,
    pub args: Vec<ScriptArgument>,
    pub delay: i32,
    pub last_int: i32,
}

impl EntityQueueRequest {
    pub fn new(queue_type: QueueType, script: ScriptFile, args: Vec<ScriptArgument>, delay: i32) -> Self {
        EntityQueueRequest {
            key: 0,
            next: None,
            prev: None,
            
            queue_type,
            script,
            args,
            delay,
            last_int: 0,
        }
    }
    
    fn unlink(&mut self) {
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