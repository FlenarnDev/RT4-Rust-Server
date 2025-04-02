use crate::entity::player::Player;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::no_timeout::NoTimeout;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub struct NoTimeoutHandler;

impl MessageHandler<NoTimeout> for NoTimeoutHandler {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }

    fn handle(&self, _: NoTimeout, _: &mut Player) -> bool {
        true
    }
}