use std::cell::{Ref, RefCell};
use std::collections::{HashMap};
use std::io::Read;
use std::net::{Shutdown, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use log::{debug, error, info};
use constants::login_out::login_out;
use constants::login_out::login_out::OK;
use constants::title_protocol::title_protocol;
use io::client_state::ConnectionState;
use io::rsa::rsa;
use crate::engine_stat::EngineStat;
use crate::entity::network_player::is_client_connected;
use crate::entity::npc::Npc;
use crate::entity::player::Player;
use crate::game_connection::GameConnection;
use crate::grid::coord_grid::CoordGrid;

pub struct EngineTick {
    pub current_tick: u32,
}

impl EngineTick {
    pub fn new() -> EngineTick {
        EngineTick { current_tick: 0, }
    }
    
    pub fn increment(&mut self) {
        self.current_tick += 1;
    }
}

pub struct Engine {
    pub members: bool,
    pub current_tick: EngineTick, // Current tick of the game world
    pub tick_rate: Duration, // TODO - make variable to support increased rate during shutdown.
    // TODO - cache?
    // TODO - ops?
    pub cycle_stats: Vec<Duration>,
    pub last_cycle_stats: Vec<Duration>,
    pub players: HashMap<i32, RefCell<Player>>,
    pub npcs: HashMap<i32, RefCell<Npc>>,
    pub new_players: Arc<Mutex<Vec<Player>>>,
    // TODO - game_map
    // TODO - zone_tracking
}

impl Engine {
    const MAX_PLAYERS: usize = 2048;
    const MAX_NPCS: usize = 8192;
    
    pub fn new() -> Engine {
        Engine { 
            members: false,
            current_tick: EngineTick::new(),
            tick_rate: Duration::from_millis(600),
            cycle_stats: vec![Duration::new(0, 0); 12],
            last_cycle_stats: vec![Duration::new(0, 0); 12],
            players: HashMap::with_capacity(Engine::MAX_PLAYERS - 1),
            npcs: HashMap::with_capacity(Engine::MAX_NPCS - 1),
            new_players: Default::default(),
        }
    }
    
    // TODO - mock function?
    
    pub fn start(&mut self, start_cycle: bool) {
        info!("Starting server on port 40001");
        let listen_addr = "127.0.0.1:40001";
        
        
        let thread_new_players = Arc::clone(&self.new_players);
        
        thread::spawn(move || {
            match TcpListener::bind(listen_addr) {
                Ok(listener) => {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                for i in 0..100 {
                                    let player = Player::new(CoordGrid::from(3094, 0, 3106), 0);
                                    // TODO: Set player properties based on connection data

                                    // Add to new_players queue
                                    let mut players_lock = thread_new_players.lock().unwrap();
                                    players_lock.push(player)
                                };
                                let mut game_connection = GameConnection::new(stream);

                                loop {

                                    if game_connection.state == ConnectionState::New {
                                        Self::on_new_connection(&mut game_connection);
                                    }

                                    debug!("Connection state: {:?}", game_connection.state);

                                    if game_connection.state == ConnectionState::Login || game_connection.state == ConnectionState::Reconnect{
                                        Self::on_login(&mut game_connection);
                                    }
                                    
                                    if game_connection.state == ConnectionState::Connected {
                                        //debug!("Connection state: {:?}", game_connection.state);
                                    }
                                    
                                }
                            }
                            Err(e) => {
                                error!("Connection failed: {}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to bind to {}: {}", listen_addr, e);
                }
            }
        });
        
        // TODO - load map
        info!("World ready!");
        if start_cycle {
            self.cycle();
        }
    }
    
    #[rustfmt::skip]
    fn cycle(&mut self) {
        loop {
            let start: Instant = Instant::now();
            
            self.process_world();
            self.process_in();
            self.process_npcs();
            self.process_players();
            self.process_logouts();
            self.process_logins();
            self.process_zones();
            self.process_movement_dirs();
            self.process_out();
            
            // Update stats
            self.cycle_stats[EngineStat::Cycle as usize] = start.elapsed();
            self.last_cycle_stats[EngineStat::Cycle as usize] = self.cycle_stats[EngineStat::Cycle as usize];
            self.last_cycle_stats[EngineStat::World as usize] = self.cycle_stats[EngineStat::World as usize];
            self.last_cycle_stats[EngineStat::ClientsIn as usize] = self.cycle_stats[EngineStat::ClientsIn as usize];
            self.last_cycle_stats[EngineStat::Npcs as usize] = self.cycle_stats[EngineStat::Npcs as usize];
            self.last_cycle_stats[EngineStat::Players as usize] = self.cycle_stats[EngineStat::Players as usize];
            self.last_cycle_stats[EngineStat::Logouts as usize] = self.cycle_stats[EngineStat::Logouts as usize];
            self.last_cycle_stats[EngineStat::Logins as usize] = self.cycle_stats[EngineStat::Logins as usize];
            self.last_cycle_stats[EngineStat::Zones as usize] = self.cycle_stats[EngineStat::Zones as usize];
            self.last_cycle_stats[EngineStat::ClientsOut as usize] = self.cycle_stats[EngineStat::ClientsOut as usize];
            self.last_cycle_stats[EngineStat::Cleanup as usize] = self.cycle_stats[EngineStat::Cleanup as usize];
            
            info!(
                "Tick: {} took: {:?} with: {} players",
                self.current_tick.current_tick,
                self.cycle_stats[EngineStat::Cycle as usize],
                self.players.len()
            );
            
            // Cycle the world now
            self.current_tick.increment();
            
            sleep(self.tick_rate.saturating_sub(start.elapsed()));
        }
    }
    
    /// - World Queue
    /// - NPC Spawn script
    /// - NPC Hunt
    fn process_world(&mut self) {
        let start: Instant = Instant::now();
        // TODO

        // NPC [ai_spawn] scripts
        // NPC hunt players if not busy
        for (_, npc) in &self.npcs {
            // Check if npc is active
            if npc.borrow().entity.active {
                // Hunts will process even if the npc is delayed during this portion
                // TODO
            }
        }

        self.cycle_stats[EngineStat::World as usize] = start.elapsed();
    }

    /// - Calculate AFK event readiness
    /// - Process Packets
    /// - Process pathfinding/following
    /// - Client input tracking
    fn process_in(&mut self) {
        let start: Instant = Instant::now();
        
        // TODO - separate out stat?
        //self.cycle_stats[EngineStat::BandwidthIn as usize] = 0;

        for (_, player) in &self.players {
            player.borrow_mut().playtime += 1;
            
            if is_client_connected(&*player.borrow()) {
                
            }
        }
        
        // TODO - decode packets
        
        // TODO - process pathfinding/following
        self.cycle_stats[EngineStat::ClientsIn as usize] = start.elapsed();
        
        
    }
    
    /// Resume suspended script
    /// Stat regeneration
    /// Timer
    /// Queue
    /// Movement
    /// Modes
    fn process_npcs(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.cycle_stats[EngineStat::Npcs as usize] = start.elapsed();
    }
    
    /// Resume suspended scripts
    /// Primary queue
    /// Weak queue
    /// Timers
    /// Soft timers
    /// Engine queue
    /// Interactions
    /// Movements
    /// Close interface if attempting to logout
    fn process_players(&mut self) {
        let start: Instant = Instant::now();
        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            // TODO
        }
        self.cycle_stats[EngineStat::Players as usize] = start.elapsed();
    }
    
    /// Player logouts
    fn process_logouts(&mut self) {
        let start: Instant = Instant::now();
        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            // TODO
        }
        self.cycle_stats[EngineStat::Logouts as usize] = start.elapsed();
    }
    
    /// Player logins
    /// Before packets so they immediately load, but after processing so nothing hits them.
    fn process_logins(&mut self) {
        let start: Instant = Instant::now();

        let player_to_add = {
            let mut shared_players = self.new_players.lock().unwrap();
            shared_players.drain(..).collect::<Vec<Player>>()
        };
        
        for player in player_to_add {
            // Prevent logging in if a player save is being flushed
            // TODO
            
            // Reconnect a new socket with player in the world
            // TODO
            
            // Player already logged in
            for (_, other_player) in &self.players {
                // TODO
            }
            
            // Prevent logging in when the server is shutting down.
            // TODO
            
            let pid: i32;
            // Check if pid available, otherwise force disconnect, world full.
            // TODO
            
            pid = (self.players.len() + 1) as i32;
            //let player = player.into_inner();
            //player.pid = pid; TODO
            self.players.insert(pid, RefCell::new(player));
        }
        self.new_players.lock().unwrap().clear();
        self.cycle_stats[EngineStat::Logins as usize] = start.elapsed();
        debug!("Processed logins in: {:?}", self.cycle_stats[EngineStat::Logins as usize])
    }
    
    /// Build list of active zones around players
    /// [loc] & [obj] despawn/respawn
    /// Compute shared buffer
    fn process_zones(&mut self) {
        let start: Instant = Instant::now();
        let tick: u32 = self.current_tick.current_tick;
        // TODO
        self.cycle_stats[EngineStat::Zones as usize] = start.elapsed();
    }
    
    /// Convert player movements
    /// Convert npc movements
    fn process_movement_dirs(&self) {
        // TODO - add benchmark value for this?
        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            // TODO
        }
        // TODO
    }
    
    /// Map update
    /// Player info
    /// NPC info
    /// Zone updates
    /// Inventory changes
    /// Stat changes
    /// AFK zone changes
    /// Flush packets
    fn process_out(&mut self) {
        let start: Instant = Instant::now();
        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            // TODO
        }
        self.cycle_stats[EngineStat::ClientsOut as usize] = start.elapsed();
    }
    
    /// Reset zones
    /// Reset players
    /// Reset npcs
    /// Reset inventories
    fn process_cleanup(&mut self) {
        let start: Instant = Instant::now();
        
        let tick = self.current_tick.current_tick;
        
        // Reset zones
        // TODO
        
        // Reset players
        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            // TODO
        }
        // Reset npcs
        // TODO
        // Reset inventories
        // TODO
        self.cycle_stats[EngineStat::Cleanup as usize] = start.elapsed();
    }
    
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.uid, RefCell::new(player));
    }
    
    pub fn get_player(&self, uid: i32) -> Result<Ref<Player>, String> {
        match self.players.get(&uid) {
            None => Err(format!("Player with uid {} not found in engine", uid)),
            Some(player_ref) => Ok(player_ref.borrow())
        }
    }

    fn on_new_connection(connection: &mut GameConnection) {
        let start: Instant = Instant::now();
        //debug!("New connection from: {}", connection.stream.peer_addr().unwrap());

        if connection.state != ConnectionState::New {
            debug!("Connection already established, closing");
            connection.shutdown();
            return
        }

        connection.read_packet().expect(
            "Failed to read packet from new connection");

        if connection.inbound.remaining() < 1 {
            debug!("Connection closed, no data received");
            connection.shutdown();
            return
        }

        connection.opcode = connection.inbound().g1() as i32;

        if connection.opcode == title_protocol::INIT_GAME_CONNECTION {
            // Used to load-balance.
            let username_hash = connection.inbound().g1();
            connection.outbound.p1(0);

            // Server session key for this connection, used in decrypting return values.
            let session_key: u64 = ((rand::random::<f64>() * 99999999.0) as u64) << 32 | ((rand::random::<f64>() * 99999999.0) as u64);
            connection.outbound.p8(session_key as i64);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.state = ConnectionState::Login
        }  else if connection.opcode == title_protocol::RECONNECT {
            connection.state = ConnectionState::Reconnect;
        } else if connection.opcode == title_protocol::LOGIN {
            connection.state = ConnectionState::Login;
        } else {
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        }
        debug!("Process new connection in: {:?}", start.elapsed());
    }

    fn on_login(connection: &mut GameConnection) {
        let start: Instant = Instant::now();

        if connection.state != ConnectionState::Login && connection.state != ConnectionState::Reconnect {
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        }

        connection.read_packet().unwrap();

        if connection.inbound.remaining() < 1 {
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            debug!("Connection closed, no data received");
            return
        }

        connection.opcode = connection.inbound().g1() as i32;

        if connection.opcode != title_protocol::LOGIN && connection.opcode != title_protocol::RECONNECT {
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        }

        debug!("Opcode during login: {}", connection.opcode);


        let login_packet_length = connection.inbound().g2();

        let client_version = connection.inbound().g4();

        if client_version != 530 {
            connection.outbound.p1(login_out::CLIENT_OUT_OF_DATE);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        } else {
            debug!("Client version: {}", client_version)
        }

        let bytes1 = connection.inbound().g1b();
        debug!("Bytes1: {}", bytes1);
        let adverts_suppressed = connection.inbound().g1b();
        debug!("Adverts suppressed: {}", adverts_suppressed);
        let client_signed = connection.inbound().g1b();
        debug!("Client signed: {}", client_signed);
        let display_mode = connection.inbound().g1b();
        debug!("Display mode: {}", display_mode);
        let canvas_width = connection.inbound().g2();
        debug!("Canvas width: {}", canvas_width);
        let canvas_height = connection.inbound().g2();
        debug!("Canvas height: {}", canvas_height);
        let anti_aliasing = connection.inbound().g1b();
        debug!("Anti aliasing: {}", anti_aliasing);
        let uid = connection.inbound().gbytes(24);
        debug!("UID: {:?}", uid);
        let site_settings_cookie = connection.inbound().gjstr(0);
        debug!("Site settings cookie: {}", site_settings_cookie);
        let affiliate_id = connection.inbound().g4();
        debug!("Affiliate ID: {}", affiliate_id);
        let detail_options = connection.inbound().g4();
        debug!("Detail options: {}", detail_options);
        let verify_id = connection.inbound().g2();
        debug!("Verify ID: {}", verify_id);

        let mut checksums = [0u32; 28];

        for i in 0..28 {
            checksums[i] = connection.inbound().g4() as u32;
            debug!("Checksum {}: {}", i, checksums[i]);
        }

        let rsa_block_length = connection.inbound().g1();
        debug!("RSA block length: {}", rsa_block_length);
        let mut rsa_packet_decrypted = rsa::decrypt_rsa_block(connection.inbound.clone(), rsa_block_length as usize);

        let rsa_verification = rsa_packet_decrypted.g1();
        if rsa_verification != 10 {
            debug!("RSA verification failed, value: {}", rsa_verification);
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        }

        let temp1 = rsa_packet_decrypted.g4();
        let temp2 = rsa_packet_decrypted.g4();
        let temp3 = rsa_packet_decrypted.g4();
        let temp4 = rsa_packet_decrypted.g4();

        let temp5 = rsa_packet_decrypted.g4();
        let temp6 = rsa_packet_decrypted.g4();

        let password = rsa_packet_decrypted.gjstr(0);
        debug!("Password: {}", password);

        
        if connection.opcode == title_protocol::RECONNECT {
            connection.outbound.p1(15)
        } else if connection.opcode == title_protocol::LOGIN {
            connection.outbound.p1(OK);
            connection.outbound.p1(2); // Staff mod level
            connection.outbound.p1(0);
            connection.outbound.p1(0);
            connection.outbound.p1(0);
            connection.outbound.p1(0);
            connection.outbound.p1(0);
            connection.outbound.p1(0);
            connection.outbound.p2(1);
            connection.outbound.p1(1);
            connection.outbound.p1(1);
        }
        connection.write_packet().expect("Failed to write packet to new connection");
        debug!("Process login in: {:?}", start.elapsed());
        connection.state = ConnectionState::Connected
    }
}