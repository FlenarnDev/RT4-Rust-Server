use crate::io::client::client_protocol_category::ClientProtocolCategory;

pub trait IncomingMessage {
    fn category(&self) -> ClientProtocolCategory;
}