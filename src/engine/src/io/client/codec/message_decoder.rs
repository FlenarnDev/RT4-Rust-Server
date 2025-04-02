use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

pub trait MessageDecoder<T> {
    fn length() -> i32;
    fn decode(protocol: ClientProtocol, packet: Packet) -> T;
}