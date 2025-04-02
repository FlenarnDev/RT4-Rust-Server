use constants::window_mode::window_mode;
use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::window_status::WindowStatus;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

impl MessageDecoder<WindowStatus> for WindowStatus {
    fn length() -> i32 {
        5
    }

    fn decode(_: ClientProtocol, mut packet: Packet) -> WindowStatus {
        let window_mode=  window_mode::from_i8(packet.g1b());
        let canvas_width = packet.g2() as u32;
        let canvas_height = packet.g1() as u32;
        let anti_aliasing_mode = packet.g1b() as u32;

        WindowStatus { window_mode, canvas_width, canvas_height, anti_aliasing_mode }
    }
}