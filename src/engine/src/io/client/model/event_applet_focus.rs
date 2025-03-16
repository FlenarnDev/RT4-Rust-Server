use std::any::Any;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub struct EventAppletFocusMessage {
    pub(crate) focus: bool,
}

impl IncomingMessage for EventAppletFocusMessage {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}