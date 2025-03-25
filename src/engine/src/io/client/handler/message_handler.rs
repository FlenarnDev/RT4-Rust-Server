use crate::entity::player::Player;
use crate::io::client::incoming_message::IncomingMessage;

pub trait MessageHandler: Send + Sync {
    type Message: IncomingMessage + Send + Sync;
    fn handle(&self, message: &Self::Message, player: &mut Player) -> bool;
}