use crate::packet::Packet;
use crate::server::outgoing_message::OutgoingMessage;
use crate::server::prot::server_protocol::ServerProtocol;

pub trait MessageEncoder<T: OutgoingMessage> {
    fn protocol(&self) -> ServerProtocol;
    fn encode(&self, packet: &mut Packet, message: &T);
    
    // TODO - replicate test case.
}