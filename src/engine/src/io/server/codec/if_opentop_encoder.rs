use log::debug;
use crate::io::packet::Packet;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::io::server::protocol::server_protocol::ServerProtocol;

pub struct If_OpenTopEncoder;

impl If_OpenTopEncoder {
    #[inline]
    pub fn new() -> Self { If_OpenTopEncoder }
}

impl MessageEncoder<If_OpenTop> for If_OpenTopEncoder {
    #[inline]
    fn protocol(&self) -> ServerProtocol { ServerProtocol::IF_OPENTOP }

    fn encode(&self, packet: &mut Packet, message: If_OpenTop) {
        packet.p2leadd(message.interface_id as i32);
        packet.p1add(message.interface_type as i32);
        packet.p2leadd(message.verify_id as i32);
    }
}