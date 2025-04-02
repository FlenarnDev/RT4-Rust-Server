use crate::entity::player::Player;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;

pub trait MessageHandler<T> {
    fn category(&self) -> ClientProtocolCategory;
    
    fn handle(&self, message: T, player: &mut Player) -> bool;
}