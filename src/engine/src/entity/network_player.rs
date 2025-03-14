use crate::entity::player::Player;
use crate::game_connection::GameConnection;

pub struct NetworkPlayer {
    pub(crate) player: Player,
    pub(crate) connection: GameConnection,
    /// User packet limit
    pub(crate) user_limit: u8,
    /// Client packet limit
    pub(crate) connection_limit: u8,
    pub restricted_limit: u8,

    pub user_path: Vec<i32>,
    pub op_called: bool,
}

impl NetworkPlayer {
    pub fn new(player: Player, connection: &mut Option<GameConnection>) -> NetworkPlayer {
        NetworkPlayer {
            player,
            connection: GameConnection::take_ownership(connection),
            user_limit: 0,
            connection_limit: 0,
            restricted_limit: 0,
            user_path: vec![],
            op_called: false,
        }
    }

    pub fn with_connection(player: Player, connection: GameConnection) -> NetworkPlayer {
        NetworkPlayer {
            player,
            connection,
            user_limit: 0,
            connection_limit: 0,
            restricted_limit: 0,
            user_path: Vec::with_capacity(4),
            op_called: false,
        }
    }

    pub fn is_client_connected(self: &Self) -> bool {
        self.connection.is_connection_active()
    }
}

