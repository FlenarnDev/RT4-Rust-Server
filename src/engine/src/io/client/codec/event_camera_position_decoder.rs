use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::event_camera_position::EventCameraPosition;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

impl MessageDecoder<EventCameraPosition> for EventCameraPosition {
    #[inline]
    fn length() -> i32 {
        4
    }

    fn decode(_: ClientProtocol, mut packet: Packet) -> EventCameraPosition {
        EventCameraPosition::new(
            packet.g2add(),
            packet.ig2()
        )
    }
}