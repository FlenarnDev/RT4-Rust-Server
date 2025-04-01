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
    pub npc_event_type: NpcEventType,
    
    pub script: ScriptFile,
    
    pub npc: NPC,
    
    pub linkable: Rc<RefCell<Linkable>>,
}

impl NpcEventRequest {
    pub fn new(npc_event_type: NpcEventType, script: ScriptFile, npc: NPC, ) -> Self {
        NpcEventRequest {
            npc_event_type,
            script,
            npc, 
            linkable: Rc::new(RefCell::new(Linkable::new())),
        }
    }
}