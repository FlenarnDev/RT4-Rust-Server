use log::error;
use crate::entity::player::Player;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::verification::VerificationMessage;

const EXPECTED_VERIFICATION_CODE: i32 = 1057001181;

pub struct VerificationHandler;

impl MessageHandler for VerificationHandler {
    type Message = VerificationMessage;

    fn handle(&self, message: &Self::Message, player: &mut Player) -> bool {
        if message.verification != EXPECTED_VERIFICATION_CODE {
            error!("Client failed verification check: got {} expected {}",
                  message.verification, EXPECTED_VERIFICATION_CODE);
            player.client.shutdown();
            return false;
        }

        true
    }
}