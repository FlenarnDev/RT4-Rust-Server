use crate::entity::network_player::NetworkPlayer;
use crate::io::client::incoming_message::IncomingMessage;

pub trait MessageHandler: Send + Sync {
    type Message: IncomingMessage + Send + Sync;
    fn handle(&self, message: &Self::Message, network_player: &mut NetworkPlayer) -> bool;
}