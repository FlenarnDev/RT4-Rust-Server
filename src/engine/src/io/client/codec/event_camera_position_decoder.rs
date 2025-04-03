use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::event_camera_position::EventCameraPositionMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

pub struct EventCameraPositionDecoder;

impl MessageDecoder for EventCameraPositionDecoder {
    type Message = EventCameraPositionMessage;

    fn protocol(&self) -> &ClientProtocol {
        &ClientProtocol::EVENT_CAMERA_POSITION
    }

    fn decode(&self, packet: &mut Packet, _length: usize) -> Box<Self::Message> {
        let camera_pitch = packet.g2add();
        let camera_yaw = packet.ig2();
        Box::new(EventCameraPositionMessage{camera_pitch, camera_yaw})
    }
}