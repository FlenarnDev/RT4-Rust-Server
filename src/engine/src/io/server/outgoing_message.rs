use std::cell::RefCell;
use std::fmt::Debug;
use crate::entity::network_player::NetworkPlayer;
use crate::io::server::info_message::InfoMessage;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;

pub trait OutgoingMessage: Debug + Send {
    fn priority(&self) -> ServerProtocolPriority;
    fn write_self(self: Box<Self>, player: &mut NetworkPlayer);
}

macro_rules! impl_outgoing_message {
    ($type:ty, $priority:expr) => {
        impl OutgoingMessage for $type {
            fn priority(&self) -> ServerProtocolPriority {
                $priority
            }
            
            fn write_self(self: Box<Self>, player: &mut NetworkPlayer) {
                player.write_inner(*self);
            }
        }
    };
}

impl_outgoing_message!(If_OpenSub, ServerProtocolPriority::BUFFERED);
impl_outgoing_message!(RebuildNormal, ServerProtocolPriority::IMMEDIATE);
impl_outgoing_message!(InfoMessage, ServerProtocolPriority::IMMEDIATE);