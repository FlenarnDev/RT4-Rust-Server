use std::cmp::PartialEq;
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
use crate::io::client::protocol::client_protocol_repository::{get_decoder, get_handler, ClientProtocolRepository};

pub struct NetworkPlayer {
    pub player: Player,
    pub client: GameClient,
    /// User packet limit
    pub user_limit: u8,
    /// Client packet limit
    pub client_limit: u8,
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
        if !self.client.has_available(1).unwrap() {
            return false
        }
        
        if self.client.opcode == -1 {
            self.client.read_packet_with_size(1).unwrap();
            
            if !self.client.decryptor.is_none() {
                // TODO - ISAAC stuff
            } else {
                self.client.opcode = self.client.inbound.g1() as i32;
            }

            if let Some(packet_type) = &BY_ID[self.client.opcode as usize] {
                self.client.waiting = packet_type.length;
            } else {
                println!("Unknown packet type: {}", self.client.opcode);
                self.client.opcode = -1;
                self.client.shutdown();
                return false;
            }
        }

        if self.client.waiting == -1 {
            self.client.read_packet_with_size(1).unwrap();
            self.client.waiting = self.client.inbound.g1() as i32;
        } else if self.client.waiting == -2 {
            self.client.read_packet_with_size(2).unwrap();
            self.client.waiting = self.client.inbound.g2() as i32;

            // TODO - Don't quite understand this logic, research.
            if self.client.waiting > 1600 {
                self.client.shutdown();
                return false;
            }
        }

        if !self.client.has_available(self.client.waiting as usize).unwrap() {
            return false;
        }
        
        self.client.read_packet_with_size(self.client.waiting as usize).unwrap();
        let packet_type = &BY_ID[self.client.opcode as usize].clone().unwrap();
        let decoder = get_decoder(packet_type);
        
        if let Some(decoder) = decoder {
            let waiting_size = self.client.waiting as usize;
            let message = decoder.decode_erased(self.client.inbound(), waiting_size);
            let success: bool = get_handler(packet_type).map_or(false, |handler| handler.handle_erased(&*message, self));
            
            if !success {
                debug!("No handler for packet: {:?}", packet_type);
            }
            
            if success && message.category() == ClientProtocolCategory::USER_EVENT {
                self.user_limit += 1;
            } else if message.category() == ClientProtocolCategory::USER_EVENT {
                self.restricted_limit += 1;
            } else {
                self.client_limit += 1;
            }
        }

        self.client.opcode = -1;
        true
    }
    
    /// initial_login
    ///
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
        self.initial_login();
        self.rebuild_normal(false);
        
        
        self.player.entity.active = true;
        debug!("Processed on login in: {:?}", start.elapsed());
    }
    
    fn initial_login(&mut self) {
        self.client.outbound.p1(2); // Staff mod level
        self.client.outbound.p1(0); // Blackmarks?
        self.client.outbound.p1(0); // Underage (false = 0)
        self.client.outbound.p1(0); // Parental Chat consent 
        self.client.outbound.p1(0); // Parental Advert Consent
        self.client.outbound.p1(0); // Map Quick Chat
        self.client.outbound.p1(0); // Mouse Recorder
        self.client.outbound.p2(self.player.pid); // Player ID
        self.client.outbound.p1(1); // Player Member
        self.client.outbound.p1(1); // Members map
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
            || reconnect 
        {
            self.write(RebuildNormal::new(CoordGrid::zone(self.player.entity.coord.x()) as i32, CoordGrid::zone(self.player.entity.coord.z()) as i32, self.player.entity.coord.local_x(), self.player.entity.coord.local_z()));
            self.player.origin_coord.coord = self.player.entity.coord.coord;
        }
    }
    
    fn write(&mut self, message: impl OutgoingMessage + 'static) {
        if !self.is_client_connected() {
            return;
        }
        
        if (message.priority() == ServerProtocolPriority::IMMEDIATE) {
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

        if !self.client.encryptor.is_none() {
            // TODO - ISAAC stuff
        } else {
            self.client.outbound.p1(protocol.id);
        }

        encoder.unwrap().encode(&mut self.client.outbound, &message);
        self.client.write_packet().unwrap();
    }
}

impl EntityExt for NetworkPlayer {
    fn id(&self) -> usize {
        self.player.pid as usize
    }
}