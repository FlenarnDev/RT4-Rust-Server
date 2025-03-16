use crate::server::outgoing_message::OutgoingMessage;
use crate::server::prot::server_protocol_priority::ServerProtocolPriority;

#[derive(Debug)]
pub struct InfoMessage {}

impl InfoMessage {
    pub fn new() -> Self {
        InfoMessage {}
    }
    
    fn persists(&self) -> bool {
        false
    }
}

impl OutgoingMessage for InfoMessage {
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
    }
}

