use crate::entity::player::Player;
use crate::io::packet::Packet;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::protocol::server_protocol::ServerProtocol;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;
use crate::io::server::protocol::server_protocol_repository::ServerProtocolRepository;
use std::fmt::Debug;
use crate::io::server::model::message_game::Message_Game;

pub trait OutgoingMessage: Debug + Send + PartialEq {
    fn priority(&self) -> ServerProtocolPriority;
    fn write_self(&self, player: &mut Player);
}

macro_rules! define_outgoing_messages {
    (
        $(($variant:ident, $type:ty, $priority:expr)),*
    ) => {
        // Implement OutgoingMessage for each type
        $(
            impl OutgoingMessage for $type {
                #[inline]
                fn priority(&self) -> ServerProtocolPriority {
                    $priority
                }

                #[inline]
                fn write_self(&self, player: &mut Player) {
                    if !player.is_client_connected() {
                        return;
                    }

                    // Get protocol from repository
                    let protocol = match player.get_server_protocol_repository().get_encoder(self) {
                        Some(encoder) => encoder.protocol(),
                        None => return,
                    };

                    // Set protocol ID
                    if player.client.encryptor.is_some() {
                        // TODO - ISAAC handling 
                    } else {
                        player.client.outbound.p1(protocol.id);
                    }

                    // Encode message directly
                    if let Some(encoder) = player.get_server_protocol_repository().get_encoder(self) {
                        encoder.encode(&mut player.client.outbound, self.clone());
                        player.client.write_packet().expect("Write packet failed");
                    }
                }
            }
        )*

        #[derive(Debug, Clone, PartialEq)]
        pub enum OutgoingMessageEnum {
            $($variant($type)),*
        }

        impl OutgoingMessage for OutgoingMessageEnum {
            #[inline]
            fn priority(&self) -> ServerProtocolPriority {
                match self {
                    $(OutgoingMessageEnum::$variant(msg) => msg.priority()),*
                }
            }

            #[inline]
            fn write_self(&self, player: &mut Player) {
                match self {
                    $(OutgoingMessageEnum::$variant(msg) => msg.write_self(player)),*
                }
            }
        }

        impl OutgoingMessageEnum {
            pub fn get_protocol(&self, repo: &ServerProtocolRepository) -> Option<ServerProtocol> {
                match self {
                    $(OutgoingMessageEnum::$variant(msg) => repo.get_encoder(msg).map(|e| e.protocol())),*
                }
            }

            pub fn encode_to_packet(&self, packet: &mut Packet, repo: &ServerProtocolRepository) -> bool {
                match self {
                    $(
                        OutgoingMessageEnum::$variant(msg) => {
                            if let Some(encoder) = repo.get_encoder(msg) {
                                encoder.encode(packet, msg.clone());
                                true
                            } else {
                                false
                            }
                        }
                    ),*
                }
            }
        }

        // Generate From implementations
        $(
            impl From<$type> for OutgoingMessageEnum {
                fn from(msg: $type) -> Self {
                    OutgoingMessageEnum::$variant(msg)
                }
            }
        )*
    };
}

// Apply the unified macro to all message types
define_outgoing_messages!(
    (RebuildNormal, RebuildNormal, ServerProtocolPriority::IMMEDIATE),
    (MessageGame, Message_Game, ServerProtocolPriority::IMMEDIATE),
    (IfOpenTop, If_OpenTop, ServerProtocolPriority::BUFFERED),
    (IfOpenSub, If_OpenSub, ServerProtocolPriority::BUFFERED)
);