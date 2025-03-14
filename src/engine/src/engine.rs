use std::cell::RefCell;
use std::collections::{HashMap};
use std::net::{IpAddr, TcpListener};
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
use crate::entity::entity_list::NetworkPlayerList;
use crate::entity::network_player::NetworkPlayer;
use crate::entity::npc::Npc;
use crate::entity::player::Player;
use crate::game_connection::GameClient;
use crate::grid::coord_grid::CoordGrid;

pub struct Engine {
    pub members: bool,
    pub current_tick: i32, // Current tick of the game world
    pub tick_rate: Duration, // TODO - make variable to support increased rate during shutdown.
    // TODO - cache?
    // TODO - ops?
    pub cycle_stats: Vec<Duration>,
    pub last_cycle_stats: Vec<Duration>,
    pub players: NetworkPlayerList,
    pub npcs: HashMap<i32, RefCell<Npc>>,
    pub new_players: Arc<Mutex<Vec<NetworkPlayer>>>,
    // TODO - game_map
    // TODO - zone_tracking
}

impl Engine {
    const MAX_PLAYERS: usize = 2048;
    const MAX_NPCS: usize = 8192;

    const TIMEOUT_NO_CONNECTION: i32 = 50;
    const TIMEOUT_NO_RESPONSE: i32 = 100;

    const AFK_EVENTRATE: i32 = 500;
    
    pub fn new() -> Engine {
        Engine { 
            members: false,
            current_tick: 0,
            tick_rate: Duration::from_millis(600),
            cycle_stats: vec![Duration::new(0, 0); 12],
            last_cycle_stats: vec![Duration::new(0, 0); 12],
            players: NetworkPlayerList::new(Engine::MAX_PLAYERS - 1),
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

                                let thread_player = Arc::clone(&thread_new_players);

                                thread::spawn(move || {
                                    let mut game_connection = GameClient::new(stream);

                                    loop {
                                        if game_connection.state == ConnectionState::New && game_connection.is_connection_active() {
                                            Self::on_new_connection(&mut game_connection, Arc::clone(&thread_player));
                                        } else {
                                            break
                                        }
                                    }
                                });
                                /*for i in 0..100 {
                                    let player = Player::new(CoordGrid::from(3094, 0, 3106), 0);
                                    // TODO: Set player properties based on connection data

                                    // Add to new_players queue
                                    let mut players_lock = thread_new_players.lock().unwrap();
                                    players_lock.push(player)
                                };*/

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
            self.process_info();
            self.process_out();
            self.process_cleanup();
            
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
                "Tick: {} took: {:?} with: {} player(s)",
                self.current_tick,
                self.cycle_stats[EngineStat::Cycle as usize],
                self.players.count()
            );
            
            // Cycle the world now
            self.current_tick += 1;
            
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
        for (nid, npc) in &self.npcs {
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
    /// - Process pathfinding/following requests
    /// - Client input tracking
    fn process_in(&mut self) {
        let start: Instant = Instant::now();
        
        // TODO - separate out stat?
        //self.cycle_stats[EngineStat::BandwidthIn as usize] = 0;

        self.players.for_each_mut(|network_player| {
            network_player.player.playtime += 1;

            if network_player.is_client_connected()  {
                if network_player.decode_in(self.current_tick) {

                }
            }
        });
        
        // TODO - decode packets
        
        // TODO - process pathfinding/following
        self.cycle_stats[EngineStat::ClientsIn as usize] = start.elapsed();
    }
    
    /// Resume suspended script
    ///
    /// Stat regeneration
    ///
    /// Timer
    ///
    /// Queue
    ///
    /// Movement
    ///
    /// Modes
    fn process_npcs(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.cycle_stats[EngineStat::Npcs as usize] = start.elapsed();
    }
    
    /// Resume suspended scripts
    ///
    /// Primary queue
    ///
    /// Weak queue
    ///
    /// Timers
    ///
    /// Soft timers
    ///
    /// Engine queue
    ///
    /// Interactions
    ///
    /// Movements
    ///
    /// Close interface if attempting to logout
    fn process_players(&mut self) {
        let start: Instant = Instant::now();
        
        self.cycle_stats[EngineStat::Players as usize] = start.elapsed();
    }
    
    /// Player logouts
    fn process_logouts(&mut self) {
        let start: Instant = Instant::now();

        let mut pids_to_remove = Vec::new();
        self.players.for_each_mut(|network_player| {
            let mut force: bool = false;

            if self.current_tick - network_player.player.last_response >= Self::TIMEOUT_NO_RESPONSE {
                // X-logged / timed out for 60s: force logout.
                debug!("X-logged");
                network_player.player.logging_out = true;
                force = true;
            } else if self.current_tick - network_player.player.last_connected >= Self::TIMEOUT_NO_CONNECTION {
                // Connection lost for 30s: request idle logout.
                debug!("idle log");
                network_player.player.request_idle_logout = true;
            }

            if network_player.player.request_logout || network_player.player.request_idle_logout {
                if self.current_tick >= network_player.player.prevent_logout_until {
                    network_player.player.logging_out = true;
                }
                network_player.player.request_logout = false;
                network_player.player.request_idle_logout = false;
            }

            if (network_player.player.logging_out) && (force || self.current_tick >= network_player.player.prevent_logout_until) {
                pids_to_remove.push(network_player.player.pid);
            }
        }) ;
        
        // TODO
        
        for pid in pids_to_remove {
            self.remove_player(pid)
        }
        
        self.cycle_stats[EngineStat::Logouts as usize] = start.elapsed();
    }
    
    /// Player logins
    ///
    /// Before packets so they immediately load, but after processing so nothing hits them.
    fn process_logins(&mut self) {
        let start: Instant = Instant::now();

        let player_to_add = {
            let mut shared_players = self.new_players.lock().unwrap();
            shared_players.drain(..).collect::<Vec<NetworkPlayer>>()
        };
        
        for mut player in player_to_add {
            debug!("Adding player!");
            // Prevent logging in if a player save is being flushed
            // TODO
            
            // Reconnect a new socket with player in the world
            // TODO
            
            // Player already logged in
            //for (_, other_player) in &self.players {
                // TODO
            //}
            
            // Prevent logging in when the server is shutting down.
            // TODO
            
            // Check if pid available, otherwise force disconnect, world full.
            // TODO
            let pid = self.get_next_pid(Some(&player.client));
            player.player.pid = pid;
            self.players.set(pid as usize, player).expect("Failed to set player!");
            
        }
        self.new_players.lock().unwrap().clear();
        self.cycle_stats[EngineStat::Logins as usize] = start.elapsed();
        debug!("Processed logins in: {:?}", self.cycle_stats[EngineStat::Logins as usize])
    }
    
    /// Build list of active zones around players
    ///
    /// [loc] & [obj] despawn/respawn
    ///
    /// Compute shared buffer
    fn process_zones(&mut self) {
        let start: Instant = Instant::now();
        let tick: u32 = self.current_tick as u32;
        // TODO
        self.cycle_stats[EngineStat::Zones as usize] = start.elapsed();
    }
    
    /// Convert player movements
    ///
    /// Compute player info
    ///
    /// Convert npc movements
    ///
    /// Compute npc info
    fn process_info(&self) {
        // TODO - add benchmark value for this?
        //for (_, player) in &self.players {
            //let _: Player = player.borrow().player;
            // TODO
       // }
        // TODO
    }
    
    /// Map update
    ///
    /// Player info
    ///
    /// NPC info
    ///
    /// Zone updates
    ///
    /// Inventory changes
    ///
    /// Stat changes
    ///
    /// AFK zone changes
    ///
    /// Flush packets
    fn process_out(&mut self) {
        let start: Instant = Instant::now();
        //for (pid, player) in &self.players {
            // TODO
        //}
        self.cycle_stats[EngineStat::ClientsOut as usize] = start.elapsed();
    }
    
    /// Reset zones
    ///
    /// Reset players
    ///
    /// Reset npcs
    ///
    /// Reset inventories
    fn process_cleanup(&mut self) {
        let start: Instant = Instant::now();
        
        let tick = self.current_tick;
        
        // Reset zones
        // TODO
        
        // Reset players
        //for (pid, player) in &self.players {
            //let _: Player = player.borrow().player;
            // TODO
        //}
        // Reset npcs
        // TODO
        // Reset inventories
        // TODO
        self.cycle_stats[EngineStat::Cleanup as usize] = start.elapsed();
    }
    
    pub fn add_player(&mut self, player: NetworkPlayer) {
        //self.players.insert(player.player.uid, RefCell::new(player));
    }
    
    /*pub fn get_player(&self, uid: i32) -> Result<Ref<Player>, String> {
        match self.players.get(&uid) {
            None => Err(format!("Player with uid {} not found in engine", uid)),
            Some(player_ref) => Ok(player_ref.borrow().player)
        }
    }*/
    
    pub fn remove_player(&mut self, pid: i32) {
        debug!("PID of player to be removed {}", pid);
        if pid == -1 {
            return;
        }
        
        if let Some(mut player_ref) = self.players.get_mut(pid as usize) {
            if player_ref.is_client_connected() {
                player_ref.client.shutdown();
            }

            player_ref.player.entity.active = false;
        }
        self.players.remove(pid as usize);
    }

    fn on_new_connection(connection: &mut GameClient, thread_player: Arc<Mutex<Vec<NetworkPlayer>>>) {
        let start: Instant = Instant::now();

        connection.read_packet_with_size(1).unwrap();

        if connection.inbound.remaining() < 1 {
            debug!("Connection closed, no data received");
            connection.shutdown();
            return
        }

        connection.opcode = connection.inbound().g1() as i32;

        if connection.opcode == title_protocol::INIT_GAME_CONNECTION {
            connection.read_packet_with_size(1).unwrap();

            // Used to load-balance.
            let _username_hash = connection.inbound().g1();
            connection.outbound.p1(0);

            // Server session key for this connection, used in decrypting return values.
            let session_key: u64 = ((rand::random::<f64>() * 99999999.0) as u64) << 32 | ((rand::random::<f64>() * 99999999.0) as u64);
            connection.outbound.p8(session_key as i64);
            connection.write_packet().expect("Failed to write packet to new connection");
        }  else if connection.opcode == title_protocol::RECONNECT || connection.opcode == title_protocol::LOGIN {
            // RECONNECT & LOGIN packet length is variable, length indicated by 'short' after opcode.
            connection.read_packet_with_size(2).unwrap();
            let payload_length = connection.inbound.g2();
            connection.read_packet_with_size(payload_length as usize).unwrap();

            let client_revision = connection.inbound.g4();
            if client_revision != 530 {
                connection.outbound.p1(login_out::CLIENT_OUT_OF_DATE);
                connection.write_packet().expect("Failed to write packet to new connection");
                connection.shutdown();
                return
            }

            let bytes1 = connection.inbound().g1b();
            //debug!("Bytes1: {}", bytes1);
            let adverts_suppressed = connection.inbound().g1b();
            //debug!("Adverts suppressed: {}", adverts_suppressed);
            let client_signed = connection.inbound().g1b();
            //debug!("Client signed: {}", client_signed);
            let display_mode = connection.inbound().g1b();
            //debug!("Display mode: {}", display_mode);
            let canvas_width = connection.inbound().g2();
            //debug!("Canvas width: {}", canvas_width);
            let canvas_height = connection.inbound().g2();
            //debug!("Canvas height: {}", canvas_height);
            let anti_aliasing = connection.inbound().g1b();
            //debug!("Anti aliasing: {}", anti_aliasing);
            let uid = connection.inbound().gbytes(24);
            //debug!("UID: {:?}", uid);
            let site_settings_cookie = connection.inbound().gjstr(0);
            //debug!("Site settings cookie: {}", site_settings_cookie);
            let affiliate_id = connection.inbound().g4();
            //debug!("Affiliate ID: {}", affiliate_id);
            let detail_options = connection.inbound().g4();
            //debug!("Detail options: {}", detail_options);
            let verify_id = connection.inbound().g2();
            //debug!("Verify ID: {}", verify_id);

            let mut checksums = [0u32; 28];

            for i in 0..28 {
                checksums[i] = connection.inbound().g4() as u32;
                // TODO - validate against server cache
                //debug!("Checksum {}: {}", i, checksums[i]);
            }

            let rsa_block_length = connection.inbound().g1();
            let mut rsa_packet_decrypted = rsa::decrypt_rsa_block(connection.inbound.clone(), rsa_block_length as usize);

            let rsa_verification = rsa_packet_decrypted.g1();
            if rsa_verification != 10 {
                debug!("RSA verification failed, received value: {}", rsa_verification);
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
            connection.state = ConnectionState::Connected;

            let mut option_connection = Some(std::mem::replace(
                connection,
                GameClient::new_dummy()
            ));

            let player = Player::new(CoordGrid::from(3094, 0, 3106), 0, 12);

            let network_player = NetworkPlayer::new(
                player,
                &mut option_connection
            );
            let mut players_lock = thread_player.lock().unwrap();
            players_lock.push(network_player);
        } else {
            connection.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            connection.write_packet().expect("Failed to write packet to new connection");
            connection.shutdown();
            return
        }
        debug!("[on_new_connection] took: {:?}", start.elapsed());
    }

    fn get_next_pid(&self, client: Option<&GameClient>) -> i32 {
        // Default case - no client or any error case
        let default = || self.players.next(false, None).unwrap_or(0) as i32;

        // Early return if no client
        let client = match client {
            Some(c) => c,
            None => return default(),
        };

        // Early return if no stream
        let stream = match &client.stream {
            Some(s) => s,
            None => return default(),
        };

        // Early return if can't get peer address
        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(_) => return default(),
        };

        // Handle different IP types
        match peer_addr.ip() {
            IpAddr::V4(ipv4) => {
                let last_octet = ipv4.octets()[3];
                let start = (last_octet % 20) as usize * 100;
                self.players.next(true, Some(start)).unwrap_or(0) as i32
            },
            IpAddr::V6(ipv6) => {
                // Create a longer-lived String first
                let ip_string = ipv6.to_string();
                let third_segment = ip_string.split(':').nth(2);

                if let Some(segment) = third_segment {
                    if segment.is_empty() {
                        return default();
                    }

                    let segment_value = i32::from_str_radix(segment, 16).unwrap_or(0);
                    let start = (segment_value % 20) as usize * 100;
                    self.players.next(true, Some(start)).unwrap_or(0) as i32
                } else {
                    default()
                }
            }
        }
    }
}