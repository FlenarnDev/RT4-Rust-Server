use crate::entity::network_player::NetworkPlayer;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::window_status::WindowStatusMessage;

pub struct WindowStatusHandler;

impl MessageHandler for WindowStatusHandler {
    type Message = WindowStatusMessage;
    fn handle(&self, message: &Self::Message, network_player: &mut NetworkPlayer) -> bool {
        network_player.player.window_mode = message.window_mode;
        true
    }
}