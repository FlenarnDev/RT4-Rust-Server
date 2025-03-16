use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub trait IncomingMessage {
    
    fn category(&self) -> ClientProtocolCategory;
}