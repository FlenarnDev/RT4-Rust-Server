use crate::entity::player::Player;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::verification::Verification;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub struct VerificationHandler;

impl MessageHandler<Verification> for VerificationHandler {

    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }

    fn handle(&self, message: Verification, player: &mut Player) -> bool {
        if message.verification != 1057001181 {
            player.client.shutdown();
            return false;
        }
        true
    }
}