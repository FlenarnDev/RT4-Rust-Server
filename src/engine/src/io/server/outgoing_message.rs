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

// Implement OutgoingMessage for RebuildNormal
impl OutgoingMessage for RebuildNormal {
    #[inline]
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
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

// Implement OutgoingMessage for If_OpenTop
impl OutgoingMessage for If_OpenTop {
    #[inline]
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
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

// Implement OutgoingMessage for If_OpenSub
impl OutgoingMessage for If_OpenSub {
    #[inline]
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
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

// Implement OutgoingMessage for InfoMessage
impl OutgoingMessage for InfoMessage {
    #[inline]
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
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

// Define the OutgoingMessageEnum
#[derive(Debug, Clone, PartialEq)]
pub enum OutgoingMessageEnum {
    RebuildNormal(RebuildNormal),
    IfOpenTop(If_OpenTop),
    IfOpenSub(If_OpenSub),
    InfoMessage(InfoMessage),
}

// Implement OutgoingMessage for OutgoingMessageEnum
impl OutgoingMessage for OutgoingMessageEnum {
    #[inline]
    fn priority(&self) -> ServerProtocolPriority {
        match self {
            OutgoingMessageEnum::RebuildNormal(msg) => msg.priority(),
            OutgoingMessageEnum::IfOpenTop(msg) => msg.priority(),
            OutgoingMessageEnum::IfOpenSub(msg) => msg.priority(),
            OutgoingMessageEnum::InfoMessage(msg) => msg.priority(),
        }
    }

    #[inline]
    fn write_self(&self, player: &mut Player) {
        match self {
            OutgoingMessageEnum::RebuildNormal(msg) => msg.write_self(player),
            OutgoingMessageEnum::IfOpenTop(msg) => msg.write_self(player),
            OutgoingMessageEnum::IfOpenSub(msg) => msg.write_self(player),
            OutgoingMessageEnum::InfoMessage(msg) => msg.write_self(player),
        }
    }
}

// Maintain original methods for compatibility
impl OutgoingMessageEnum {
    pub fn get_protocol(&self, repo: &ServerProtocolRepository) -> Option<ServerProtocol> {
        match self {
            OutgoingMessageEnum::RebuildNormal(msg) => repo.get_encoder(msg).map(|e| e.protocol()),
            OutgoingMessageEnum::IfOpenTop(msg) => repo.get_encoder(msg).map(|e| e.protocol()),
            OutgoingMessageEnum::IfOpenSub(msg) => repo.get_encoder(msg).map(|e| e.protocol()),
            OutgoingMessageEnum::InfoMessage(msg) => repo.get_encoder(msg).map(|e| e.protocol()),
        }
    }

    pub fn encode_to_packet(&self, packet: &mut Packet, repo: &ServerProtocolRepository) -> bool {
        match self {
            OutgoingMessageEnum::RebuildNormal(msg) => {
                if let Some(encoder) = repo.get_encoder(msg) {
                    encoder.encode(packet, msg.clone());
                    true
                } else {
                    false
                }
            },
            OutgoingMessageEnum::IfOpenTop(msg) => {
                if let Some(encoder) = repo.get_encoder(msg) {
                    encoder.encode(packet, msg.clone());
                    true
                } else {
                    false
                }
            },
            OutgoingMessageEnum::IfOpenSub(msg) => {
                if let Some(encoder) = repo.get_encoder(msg) {
                    encoder.encode(packet, msg.clone());
                    true
                } else {
                    false
                }
            },
            OutgoingMessageEnum::InfoMessage(msg) => {
                if let Some(encoder) = repo.get_encoder(msg) {
                    encoder.encode(packet, msg.clone());
                    true
                } else {
                    false
                }
            },
        }
    }
}

// From implementations for message types to OutgoingMessageEnum
impl From<RebuildNormal> for OutgoingMessageEnum {
    fn from(msg: RebuildNormal) -> Self {
        OutgoingMessageEnum::RebuildNormal(msg)
    }
}

impl From<If_OpenTop> for OutgoingMessageEnum {
    fn from(msg: If_OpenTop) -> Self {
        OutgoingMessageEnum::IfOpenTop(msg)
    }
}

impl From<If_OpenSub> for OutgoingMessageEnum {
    fn from(msg: If_OpenSub) -> Self {
        OutgoingMessageEnum::IfOpenSub(msg)
    }
}

impl From<InfoMessage> for OutgoingMessageEnum {
    fn from(msg: InfoMessage) -> Self {
        OutgoingMessageEnum::InfoMessage(msg)
    }
}