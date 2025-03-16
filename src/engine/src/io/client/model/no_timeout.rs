use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;
use crate::io::client::incoming_message::IncomingMessage;

pub struct NoTimeout;

impl IncomingMessage for NoTimeout {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }
}