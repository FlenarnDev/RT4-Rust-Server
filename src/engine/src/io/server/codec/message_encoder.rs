use crate::io::packet::Packet;
use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::prot::server_protocol::ServerProtocol;

pub trait MessageEncoder<T: OutgoingMessage> {
    fn protocol(&self) -> ServerProtocol;
    fn encode(&self, packet: &mut Packet, message: &T);
    
    // TODO - replicate test case.
}