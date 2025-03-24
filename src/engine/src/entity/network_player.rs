use std::time::Instant;
use log::{debug, error};
use crate::engine::Engine;
use crate::io::client::protocol::client_protocol::BY_ID;
use crate::io::client::protocol::client_protocol_category::ClientProtocolCategory;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::protocol::server_protocol::ServerProtocol;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;
use crate::io::server::protocol::server_protocol_repository::SERVER_PROTOCOL_REPOSITORY;
use crate::entity::entity::EntityBehavior;
use crate::entity::player::Player;
use crate::game_connection::GameClient;
use crate::grid::coord_grid::CoordGrid;
use crate::io::client::protocol::client_protocol_repository::{get_decoder, get_handler};
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::script::script_provider::ScriptProvider;
use crate::script::script_runner::ScriptRunner;
use crate::script::server_trigger_types::ServerTriggerTypes;

pub struct NetworkPlayer {
    pub player: Player,
    pub client: GameClient,
    /// User packet limit
    pub user_limit: u8,
    /// Client packet limit
    pub client_limit: u8,
    pub restricted_limit: u8,

    pub outgoing_messages: Vec<Box<dyn OutgoingMessage>>,
    
    pub user_path: Vec<i32>,
    pub op_called: bool,
    pub bytes_read: usize,
}

impl NetworkPlayer {
    pub fn new(player: Player, client: &mut Option<GameClient>) -> NetworkPlayer {
        NetworkPlayer {
            player,
            client: GameClient::take_ownership(client),
            user_limit: 0,
            client_limit: 0,
            restricted_limit: 0,
            outgoing_messages: Vec::new(),
            user_path: Vec::new(),
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
            outgoing_messages: Vec::new(),
            user_path: Vec::new(),
            op_called: false,
            bytes_read: 0,
        }
    }

    #[inline]
    pub fn is_client_connected(self: &Self) -> bool {
        self.client.is_connection_active()
    }
    
    pub fn decode_in(&mut self, current_tick: i32) -> bool {
        self.user_path.clear();
        self.op_called = false;
        
        if !self.is_client_connected() {
            return false;
        }
        
        self.player.last_connected = current_tick;
        
        self.user_limit = 0;
        self.client_limit = 0;
        self.restricted_limit = 0;
        
        while self.user_limit < ClientProtocolCategory::USER_EVENT.limit && self.client_limit < ClientProtocolCategory::CLIENT_EVENT.limit && self.restricted_limit < ClientProtocolCategory::RESTRICTED_EVENT.limit && self.read() {}
        
        if self.bytes_read > 0 {
            self.player.last_response = current_tick;
            self.bytes_read = 0;
        }
        
        true
    }
    
    pub fn encode_out(&mut self, current_tick: i32) {
        if !self.is_client_connected() {
            return;
        }
        
        // TODO - modal refresh!

        let messages = std::mem::take(&mut self.outgoing_messages);
        for message in messages {
            message.write_self(self);
        }
        self.outgoing_messages.clear();
    }
    
    #[inline]
    fn read(&mut self) -> bool {
        if !self.client.has_available(1).unwrap() {
            return false
        }
        
        if self.client.opcode == 0 {
            self.client.read_packet_with_size(1).unwrap();
            
            if !self.client.decryptor.is_none() {
                // TODO - ISAAC stuff
            } else {
                self.client.opcode = self.client.inbound.g1();
            }

            if let Some(packet_type) = &BY_ID[self.client.opcode as usize] {
                self.client.waiting = packet_type.length;
            } else {
                debug!("Unknown packet type received: {}", self.client.opcode);
                self.client.opcode = 0;
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
            } else if message.category() == ClientProtocolCategory::RESTRICTED_EVENT {
                self.restricted_limit += 1;
            } else {
                self.client_limit += 1;
            }
        }
        
        self.bytes_read += self.client.inbound.position;

        self.client.opcode = 0;
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
        self.initial_login_data();
        self.rebuild_normal(false);
        
        // Initial interface settings.
        let window_id = if self.player.window_status.window_mode.is_resizeable() { 746 } else { 548 };
        let mut verify_id = self.player.get_incremented_verify_id();
       
        self.write(If_OpenTop::new(window_id, false, verify_id));
        verify_id = self.player.get_incremented_verify_id();
        self.write(If_OpenSub::new(window_id, 100, 662, 1, verify_id));
        //self.write(If_OpenSub::new(752, 8, 137, 0, self.player.get_incremented_verify_id()));
        
        let login_trigger = ScriptProvider::get_by_trigger_specific(ServerTriggerTypes::LOGIN, -1, -1);
        if let Some(trigger) = login_trigger {
            self.player.execute_script(ScriptRunner::init(trigger, Some(self.player.clone().as_entity_type()), None, None), Some(true), None)
        } else {
            debug!("Login triggered but not found");
        }
        
        // TODO - last step
        self.player.set_active(true);
        debug!("Processed on login in: {:?}", start.elapsed());
    }

    fn initial_login_data(&mut self) {
        self.client.outbound.p1(self.player.get_staff_mod_level()); // Staff mod level
        self.client.outbound.p1(0); // Blackmarks?
        self.client.outbound.p1(0); // Underage (false = 0)
        self.client.outbound.p1(0); // Parental Chat consent
        self.client.outbound.p1(0); // Parental Advert Consent
        self.client.outbound.p1(0); // Map Quick Chat
        self.client.outbound.p1(0); // Mouse Recorder
        self.client.outbound.p2(self.player.get_pid() as i32); // Player ID
        self.client.outbound.p1(1); // Player Member
        self.client.outbound.p1(1); // Members map
    }
    
    fn rebuild_normal(&mut self, reconnect: bool) {
        let origin_x = CoordGrid::zone(self.player.get_origin_coord().x()) as i16;
        let origin_z = CoordGrid::zone(self.player.get_origin_coord().z()) as i16;
        
        let reload_left_x = (origin_x - 4) << 3;
        let reload_right_x = (origin_x + 5) << 3;
        let reload_top_z = (origin_z + 5) << 3;
        let reload_bottom_z = (origin_z - 4) << 3;
        
        // If the build area should be regenerated, do so now
        if self.player.coord().x() < reload_left_x as u16
            || self.player.coord().z() < reload_bottom_z as u16
            || self.player.coord().x() > (reload_right_x - 1) as u16
            || self.player.coord().z() > (reload_top_z - 1) as u16
            || reconnect
        {
            self.write(RebuildNormal::new(CoordGrid::zone(self.player.get_coord().x()) as i32, CoordGrid::zone(self.player.get_coord().z()) as i32, self.player.get_coord().local_x(), self.player.get_coord().local_z()));
            self.player.set_origin_coord(self.player.get_coord());
        }
    }
    
    fn write(&mut self, message: impl OutgoingMessage + 'static) {
        if !self.is_client_connected() {
            return;
        }
        
        if message.priority() == ServerProtocolPriority::IMMEDIATE {
            self.write_inner(message);
        } else {
            self.outgoing_messages.push(Box::new(message));
        }
    }
    
    pub(crate) fn write_inner(&mut self, message: impl OutgoingMessage + 'static) {
        if !self.is_client_connected() {
            return;
        }
        
        let encoder = match SERVER_PROTOCOL_REPOSITORY.get_encoder(&message) {
            Some(e) => e,
            None => {
                error!("No encoder found for {:?}", message);
                return
            },
        };
        
        let protocol: ServerProtocol = encoder.protocol();

        if self.client.encryptor.is_some() {
            // TODO - ISAAC stuff
        } else {
            self.client.outbound.p1(protocol.id);
        }

        encoder.encode(&mut self.client.outbound, &message);
        self.client.write_packet().unwrap();
    }
}