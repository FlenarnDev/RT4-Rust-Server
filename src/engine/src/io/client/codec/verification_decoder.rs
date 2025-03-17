use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::verification::VerificationMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

pub struct VerificationDecoder;

impl MessageDecoder for VerificationDecoder {
    type Message = VerificationMessage;

    fn protocol(&self) -> &ClientProtocol {
        &ClientProtocol::VERIFICATION
    }

    fn decode(&self, packet: &mut Packet, _length: usize) -> Box<Self::Message> {
        let verification = packet.g4();
        Box::new(VerificationMessage{verification})
    }
}