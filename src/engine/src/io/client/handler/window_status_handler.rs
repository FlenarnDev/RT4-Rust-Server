use crate::entity::player::Player;
use crate::entity::window_status::WindowStatus;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::model::window_status::WindowStatusMessage;

pub struct WindowStatusHandler;

impl MessageHandler for WindowStatusHandler {
    type Message = WindowStatusMessage;
    fn handle(&self, message: &Self::Message, player: &mut Player) -> bool {
        player.window_status = 
            WindowStatus::new(
                message.window_mode, 
                message.canvas_width, 
                message.canvas_height, 
                message.anti_aliasing_mode
            );
        true
    }
}