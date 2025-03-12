use crate::entity::player::Player;

pub struct NetworkPlayer {
    player: Player,
    /// User packet limit
    user_limit: u8, 
    /// Client packet limit
    client_limit: u8,
    restricted_limit: u8,
    
    user_patch: Vec<i32>,
    opcalled: bool,
    
}
