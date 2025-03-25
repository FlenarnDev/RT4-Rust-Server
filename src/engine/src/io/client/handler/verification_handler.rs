use crate::entity::player::Player;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::verification::VerificationMessage;

pub struct VerificationHandler;

impl MessageHandler for VerificationHandler {
    type Message = VerificationMessage;

    fn handle(&self, message: &Self::Message, player: &mut Player) -> bool {
        if message.verification != 1057001181 {
            player.client.shutdown();
            return false;
        }
            
        true
    }
}