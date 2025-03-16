use std::fmt::Debug;
use crate::server::prot::server_protocol_priority::ServerProtocolPriority;

pub trait OutgoingMessage: Debug {
    fn priority(&self) -> ServerProtocolPriority;
}