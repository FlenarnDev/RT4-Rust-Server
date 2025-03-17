use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;

#[derive(Debug)]
pub struct If_OpenSub {
    component: u32,
    reset_worldmap: u8
}

impl If_OpenSub {
    pub fn new(component: u32, reset_worldmap: bool) -> If_OpenSub {
        If_OpenSub { component, reset_worldmap: if reset_worldmap { RESET_WORLDMAP } else { DEFAULT }}
    }
}

pub const DEFAULT: u8 = 0;
pub const RESET_WORLDMAP: u8 = 2;

impl OutgoingMessage for If_OpenSub {
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::BUFFERED
    }
}