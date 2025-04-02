use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::no_timeout::NoTimeout;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

impl MessageDecoder<NoTimeout> for NoTimeout {
    fn length() -> i32 {
        0
    }

    fn decode(_: ClientProtocol, _: Packet) -> NoTimeout {
        NoTimeout::DEFAULT
    }
}