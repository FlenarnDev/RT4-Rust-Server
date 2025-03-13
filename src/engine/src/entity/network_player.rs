use crate::entity::player::Player;
use crate::game_connection::GameConnection;

pub struct NetworkPlayer {
    player: Player,
    client: Box<GameConnection>,
    /// User packet limit
    user_limit: u8, 
    /// Client packet limit
    client_limit: u8,
    restricted_limit: u8,
    
    user_patch: Vec<i32>,
    opcalled: bool,
}

pub fn is_client_connected(player: &Player) -> bool {
    if let Some(network_player) = player.as_any().downcast_ref::<NetworkPlayer>() {
        !network_player.client.is_connection_active()
    } else {
        false
    }
}
