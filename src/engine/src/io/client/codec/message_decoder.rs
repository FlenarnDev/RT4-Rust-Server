use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::packet::Packet;

pub trait MessageDecoder {
    fn protocol(&self) -> &ClientProtocol;
    fn decode(self, packet: &Packet, length: usize) -> Box<dyn IncomingMessage>;
}