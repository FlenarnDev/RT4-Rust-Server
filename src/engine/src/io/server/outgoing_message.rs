use std::fmt::Debug;
use crate::entity::player::Player;
use crate::io::server::info_message::InfoMessage;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;
use crate::io::server::protocol::server_protocol_repository::ServerProtocolRepository;
use crate::io::server::protocol::server_protocol::ServerProtocol;
use crate::io::packet::Packet;

pub trait OutgoingMessage: Debug + Send + PartialEq {
    fn priority(&self) -> ServerProtocolPriority;
    fn write_self(&self, player: &mut Player);
}

macro_rules! define_outgoing_messages {
    ($( $variant:ident => $type:ty => $priority:expr ),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum OutgoingMessageEnum {
            $( $variant($type), )*
        }

        impl OutgoingMessage for OutgoingMessageEnum {
            fn priority(&self) -> ServerProtocolPriority {
                match self {
                    $(OutgoingMessageEnum::$variant(_) => $priority,)*
                }
            }

            fn write_self(&self, player: &mut Player) {
                match self {
                    $(OutgoingMessageEnum::$variant(msg) => player.write_inner(OutgoingMessageEnum::$variant(msg.clone())),)*
                }
            }
        }

        impl OutgoingMessageEnum {
            // Direct method without closures or boxing
            pub fn encode_to_packet(&self, packet: &mut Packet, repo: &ServerProtocolRepository) -> Option<ServerProtocol> {
                match self {
                    $(
                        OutgoingMessageEnum::$variant(msg) => {
                            if let Some(encoder) = repo.get_encoder(msg) {
                                encoder.encode(packet, msg.clone());
                                Some(encoder.protocol())
                            } else {
                                None
                            }
                        },
                    )*
                }
            }
        }

        $(
            impl OutgoingMessage for $type {
                fn priority(&self) -> ServerProtocolPriority {
                    $priority
                }

                fn write_self(&self, player: &mut Player) {
                    player.write_inner(OutgoingMessageEnum::$variant(self.clone()));
                }
            }

            impl From<$type> for OutgoingMessageEnum {
                fn from(msg: $type) -> Self {
                    OutgoingMessageEnum::$variant(msg)
                }
            }
        )*
    };
}

define_outgoing_messages!(
    IfOpenTop => If_OpenTop => ServerProtocolPriority::IMMEDIATE,
    IfOpenSub => If_OpenSub => ServerProtocolPriority::IMMEDIATE,
    RebuildNormal => RebuildNormal => ServerProtocolPriority::IMMEDIATE,
    InfoMessage => InfoMessage => ServerProtocolPriority::IMMEDIATE,
);
