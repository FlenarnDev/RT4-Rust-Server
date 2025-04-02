use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::verification::Verification;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

impl MessageDecoder<Verification> for Verification {
    fn length() -> i32 {
        4
    }

    fn decode(_: ClientProtocol, mut packet: Packet) -> Verification {
        Verification::new(
            packet.g4()
        )
    }
}