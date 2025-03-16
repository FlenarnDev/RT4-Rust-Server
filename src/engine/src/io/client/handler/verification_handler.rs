use crate::entity::network_player::NetworkPlayer;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::verification::VerificationMessage;

pub struct VerificationHandler;

impl MessageHandler for VerificationHandler {
    type Message = VerificationMessage;

    fn handle(&self, message: &Self::Message, network_player: &mut NetworkPlayer) -> bool {
        if message.verification != 1057001181 {
            network_player.client.shutdown();
            return false;
        }
            
        true
    }
}