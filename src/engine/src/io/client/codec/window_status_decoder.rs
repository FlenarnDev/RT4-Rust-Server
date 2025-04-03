use constants::window_mode::window_mode;
use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::window_status::WindowStatusMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

pub struct WindowStatusDecoder;

impl MessageDecoder for WindowStatusDecoder {
    type Message = WindowStatusMessage;

    fn protocol(&self) -> &ClientProtocol {
        &ClientProtocol::WINDOW_STATUS
    }

    fn decode(&self, packet: &mut Packet, _length: usize) -> Box<Self::Message> {
        let window_mode=  window_mode::from_i8(packet.g1b());
        let canvas_width = packet.g2() as u32;
        let canvas_height = packet.g1() as u32;
        let anti_aliasing_mode = packet.g1b() as u32;
        Box::new(WindowStatusMessage { window_mode, canvas_width, canvas_height, anti_aliasing_mode })
    }
}