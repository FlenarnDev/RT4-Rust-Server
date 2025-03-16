use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::packet::Packet;

pub trait MessageDecoder: Send + Sync {
    type Message: IncomingMessage + Send + Sync;
    fn protocol(&self) -> &ClientProtocol;
    fn decode(&self, packet: &mut Packet, length: usize) -> Box<Self::Message>;
}