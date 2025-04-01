use crate::entity::entity_queue_request::ScriptArgument;
use crate::script::script_file::ScriptFile;

#[repr(u8)]
pub enum NPCTimerType {
    NPC,
}

#[repr(u8)]
pub enum PlayerTimerType {
    Normal,
    Soft
}

pub enum TimerType {
    Npc(NPCTimerType),
    Player(PlayerTimerType),
}

pub struct EntityTimer {
    pub timer_type: TimerType,
    pub script: ScriptFile,
    pub args: Option<Vec<ScriptArgument>>,
    pub interval: f64,
    pub clock: f64
}