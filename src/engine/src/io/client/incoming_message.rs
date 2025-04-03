use std::any::Any;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub trait IncomingMessage: Any + Send + Sync {
    
    fn category(&self) -> ClientProtocolCategory;

    fn as_any(&self) -> &dyn Any;
}