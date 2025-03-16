use std::time::Instant;
use log::{debug, error};
use crate::io::client::protocol::client_protocol::BY_ID;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::protocol::server_protocol::ServerProtocol;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;
use crate::io::server::protocol::server_protocol_repository::SERVER_PROTOCOL_REPOSITORY;
use crate::entity::entity::EntityExt;
use crate::entity::player::Player;
use crate::game_connection::GameClient;
use crate::grid::coord_grid::CoordGrid;

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
        
        if self.client.opcode == -1 {
            self.client.read_packet_with_size(1).unwrap();
            
            if !self.client.decryptor.is_none() {
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
        
        self.client.opcode = -1;
        true
    }
    
    /// rebuild_normal
    /// 
    /// chat_filter_settings
    /// 
    /// varp_reset
    /// 
    /// varps
    /// 
    /// inventories
    /// 
    /// interfaces
    /// 
    /// stats
    /// 
    /// runweight
    /// 
    /// runenergy
    /// 
    /// reset animations
    /// 
    /// social
    pub fn on_login(&mut self) {
        let start = Instant::now();
        self.rebuild_normal(false); 
        
        
        
        self.player.entity.active = true;
        debug!("Processed on login in: {:?}", start.elapsed());
    }
    
    fn rebuild_normal(&mut self, reconnect: bool) {
        let origin_x = CoordGrid::zone(self.player.origin_coord.x()) as i16;
        let origin_z = CoordGrid::zone(self.player.origin_coord.z()) as i16;
        
        let reload_left_x = (origin_x - 4) << 3;
        let reload_right_x = (origin_x + 5) << 3;
        let reload_top_z = (origin_z + 5) << 3;
        let reload_bottom_z = (origin_z - 4) << 3;
        
        // If the build area should be regenerated, do so now
        if self.player.entity.coord.x() < reload_left_x as u16
            || self.player.entity.coord.z() < reload_bottom_z as u16
            || self.player.entity.coord.x() > (reload_right_x - 1) as u16
            || self.player.entity.coord.z() > (reload_top_z - 1) as u16
            || reconnect {
            self.write(RebuildNormal::new(CoordGrid::zone(self.player.entity.coord.x()) as i32, CoordGrid::zone(self.player.entity.coord.z()) as i32, self.player.entity.coord.local_x(), self.player.entity.coord.local_z()));
            
            debug!("Rebuilding normal, player coord: {}, {}", self.player.entity.coord.x(), self.player.entity.coord.z());
            
            self.player.origin_coord.coord = self.player.entity.coord.coord;
        }
    }
    
    fn write(&mut self, message: impl OutgoingMessage + 'static) {
        if !self.is_client_connected() {
            return;
        }
        
        if (message.priority() == ServerProtocolPriority::IMMEDIATE) {
            debug!("IMMEDIATE outgoing message: {:?}", message);
            self.write_inner(message);
        } else {
            // TODO - buffer push
        }
    }
    
    fn write_inner(&mut self, message: impl OutgoingMessage + 'static) {
        if !self.is_client_connected() {
            return;
        }
        
        let encoder = SERVER_PROTOCOL_REPOSITORY.get_encoder(&message);
        
        if encoder.is_none() {
            error!("No encoder found for {:?}", message);
            return;
        }
        
        let protocol: ServerProtocol = encoder.unwrap().protocol();
        debug!("Protocol: {:?}", protocol);

        //self.client.outbound.position = 0;
        
        if !self.client.encryptor.is_none() {
            // TODO - ISAAC stuff
        } else {
            self.client.outbound.p1(protocol.id);
        }
        
        debug!("Outbound buffer size: {}", self.client.outbound().data.len());
        
        encoder.unwrap().encode(&mut self.client.outbound, &message);
        
        self.client.write_packet().unwrap();
    }
}

impl EntityExt for NetworkPlayer {
    fn id(&self) -> usize {
        self.player.pid as usize
    }
}