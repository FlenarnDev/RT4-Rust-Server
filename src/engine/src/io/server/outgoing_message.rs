use std::fmt::Debug;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;

pub trait OutgoingMessage: Debug {
    fn priority(&self) -> ServerProtocolPriority;
}