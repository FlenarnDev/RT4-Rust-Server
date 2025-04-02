use std::any::Any;
use crate::entity::player::Player;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub trait MessageHandler {
    fn category(&self) -> ClientProtocolCategory;
    
    fn handle(&self, message: &dyn Any, player: &mut Player) -> bool;
}