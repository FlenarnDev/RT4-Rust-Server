use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::entity::npc::NPC;
use crate::script::script_file::ScriptFile;
use crate::util::linkable::Linkable;

pub enum NpcEventType {
    Spawn,
    Despawn,
}

pub struct NpcEventRequest {
    pub r#type: NpcEventType,
    
    pub script: Rc<RefCell<ScriptFile>>,
    
    pub npc: Rc<RefCell<NPC>>,
    
    pub linkable: Rc<RefCell<Linkable>>,
}

impl NpcEventRequest {
    pub fn new(
        r#type: NpcEventType,
        script: Rc<RefCell<ScriptFile>>,
        npc: Rc<RefCell<NPC>>,
    ) -> Self {
        Self {
            r#type,
            script,
            npc, 
            linkable: Rc::new(RefCell::new(Linkable::new())),
        }
    }
}