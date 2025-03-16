use crate::entity::network_player::NetworkPlayer;
use crate::io::client::incoming_message::IncomingMessage;

pub trait MessageHandler<T: IncomingMessage> {
    fn handle(&self, message: &T, network_player: &NetworkPlayer);
}