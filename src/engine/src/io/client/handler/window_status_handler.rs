use crate::entity::player::Player;
use crate::entity::window_status::WindowStatus;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub struct WindowStatusHandler;

impl MessageHandler<WindowStatus> for WindowStatusHandler {
    fn category(&self) -> ClientProtocolCategory {
        ClientProtocolCategory::CLIENT_EVENT
    }

    fn handle(&self, message: WindowStatus, player: &mut Player) -> bool {
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