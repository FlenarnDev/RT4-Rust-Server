use crate::io::packet::Packet;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::protocol::server_protocol::ServerProtocol;

pub struct If_OpenSub_Encoder;

impl If_OpenSub_Encoder {
    #[inline]
    pub fn new() -> Self { If_OpenSub_Encoder }
}

impl MessageEncoder<If_OpenSub> for If_OpenSub_Encoder {
    #[inline]
    fn protocol(&self) -> ServerProtocol { ServerProtocol::IF_OPENSUB }

    fn encode(&self, packet: &mut Packet, message: If_OpenSub) {
        packet.p1(message.flags as i32);
        
        let component_pointer = message.window_id << 16 | message.component_id;
        
        packet.p4me(component_pointer as i32);
        packet.p2leadd(message.verify_id as i32);
        packet.p2(message.interface_id as i32);
    }
}