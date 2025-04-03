use std::any::Any;
use constants::window_mode::window_mode;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

#[derive(Debug)]
pub struct WindowStatusMessage {
    pub(crate) window_mode: window_mode,
    pub(crate) canvas_width: u32,
    pub(crate) canvas_height: u32,
    pub(crate) anti_aliasing_mode: u32
}

impl IncomingMessage for WindowStatusMessage {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}