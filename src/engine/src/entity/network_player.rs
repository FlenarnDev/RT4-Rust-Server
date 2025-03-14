use log::debug;
use io::client_protocol::BY_ID;
use io::client_protocol_category::ClientProtocolCategory;
use crate::entity::entity::EntityExt;
use crate::entity::player::Player;
use crate::game_connection::GameClient;

pub struct NetworkPlayer {
    pub(crate) player: Player,
    pub(crate) client: GameClient,
    /// User packet limit
    pub(crate) user_limit: u8,
    /// Client packet limit
    pub(crate) client_limit: u8,
    pub restricted_limit: u8,

    pub user_path: Vec<i32>,
    pub op_called: bool,
    pub bytes_read: i32,
}

impl NetworkPlayer {
    pub fn new(player: Player, client: &mut Option<GameClient>) -> NetworkPlayer {
        NetworkPlayer {
            player,
            client: GameClient::take_ownership(client),
            user_limit: 0,
            client_limit: 0,
            restricted_limit: 0,
            user_path: vec![],
            op_called: false,
            bytes_read: 0,
        }
    }

    pub fn with_connection(player: Player, client: GameClient) -> NetworkPlayer {
        NetworkPlayer {
            player,
            client,
            user_limit: 0,
            client_limit: 0,
            restricted_limit: 0,
            user_path: vec![],
            op_called: false,
            bytes_read: 0,
        }
    }

    pub fn is_client_connected(self: &Self) -> bool {
        self.client.is_connection_active()
    }
    
    pub fn decode_in(&mut self, current_tick: i32) -> bool {
        self.user_path = vec![];
        self.op_called = false;
        
        if !self.is_client_connected() {
            return false;
        }
        debug!("user connected");
        
        self.player.last_connected = current_tick;
        
        self.user_limit = 0;
        self.client_limit = 0;
        self.restricted_limit = 0;
        
        while self.user_limit < ClientProtocolCategory::USER_EVENT.limit && self.client_limit < ClientProtocolCategory::CLIENT_EVENT.limit && self.restricted_limit < ClientProtocolCategory::RESTRICTED_EVENT.limit && self.read() {
        }
        true
    }
    
    fn read(&mut self) -> bool {
        debug!("read");
        let mut peek_buf = [0u8; 1];

        match self.client.stream.as_ref().unwrap().set_nonblocking(true) {
            Ok(_) => {},
            Err(e) => {
                debug!("Failed to set nonblocking mode: {:?}", e);
                return false;
            }
        }
        
        match self.client.stream.as_ref().unwrap().peek(peek_buf.as_mut()) {
            Ok(0) => {
                debug!("Connection closed (0 bytes)");
                return false;
            }
            Ok(_) => {
                // Data is available, continue processing
                debug!("Data available");
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available yet, but connection is still open
                debug!("No data available yet");
                return false;
            }
            Err(e) => {
                debug!("Error peeking: {:?}", e);
                return false;
            }
        }

        match self.client.stream.as_ref().unwrap().set_nonblocking(false) {
            Ok(_) => {},
            Err(e) => {
                debug!("Failed to reset blocking mode: {:?}", e);
                return false;
            }
        }
        
        debug!("read2");
        
        if self.client.opcode == -1 {
            debug!("opcode == -1");
            self.client.read_packet_with_size(1).unwrap();
            
            if !self.client.encryptor.is_none() {
                // TODO - ISAAC stuff
            } else {
                self.client.opcode = self.client.inbound.g1() as i32;
            }

            if let Some(packet_type) = &BY_ID[self.client.opcode as usize] {
                // Use packet_type
                println!("Packet length: {:?}", packet_type);
                return true;
            } else {
                println!("Unknown packet type: {}", self.client.opcode);
                return false;
            }
        }
        debug!("tada?");
        
        true
    }
    
    fn on_login() {
        
    }
}

impl EntityExt for NetworkPlayer {
    fn id(&self) -> usize {
        self.player.pid as usize
    }
}

