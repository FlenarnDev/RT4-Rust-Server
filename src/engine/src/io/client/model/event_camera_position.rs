use std::any::Any;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub struct EventCameraPositionMessage {
    pub(crate) camera_pitch: u16,
    pub(crate) camera_yaw: u16,
}

impl IncomingMessage for EventCameraPositionMessage {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}