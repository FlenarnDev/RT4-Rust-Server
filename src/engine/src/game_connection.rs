use uuid::Uuid;
use io::connection::Connection;
use io::isaac::Isaac;
use crate::entity::network_player::NetworkPlayer;

pub struct GameConnection {
    connection: Connection,
    uuid: Uuid,
    total_bytes_read: usize,
    total_bytes_written: usize,
    player: Option<NetworkPlayer>,
    encryptor: Option<Isaac>,
    decryptor: Option<Isaac>,
    /// Current opcode being read.
    opcode: i32,
    /// Bytes to wait for (if any)
    waiting: usize,
}

impl GameConnection {
    pub fn new(connection: Connection) -> GameConnection {
        GameConnection {
            connection,
            uuid: Uuid::new_v4(),
            total_bytes_read: 0,
            total_bytes_written: 0,
            player: None,
            encryptor: None,
            decryptor: None,
            opcode: -1,
            waiting: 0,
        }
    }
}