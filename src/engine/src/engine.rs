use std::net::{IpAddr, TcpListener};
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use log::{debug, error, info};
use cache::file_handler::{ensure_initialized, get_checksum};
use cache::xtea::{initialize_xtea, XTEAKey};
use constants::window_mode::window_mode;
use constants::login_out::login_out;
use constants::title_protocol::title_protocol;
use crate::io::client_state::ConnectionState;
use crate::io::rsa::rsa;
use crate::engine_stat::engine_stat;
use crate::entity::entity::EntityBehavior;
use crate::entity::entity_list::{NPCList, PlayerList};
use crate::entity::player::Player;
use crate::entity::window_status::WindowStatus;
use crate::game_connection::GameClient;
use crate::grid::coord_grid::CoordGrid;
use crate::io::packet::Packet;
use crate::script::script_provider::ScriptProvider;
use crate::util::base37::decode37;
use crate::util::pack_file::revalidate_pack;
use crate::util::runescript_compiler::update_compiler;
use crate::util::symbols::generate_server_symbols;

pub struct Engine {
    pub members: bool,
    pub current_tick: i32,
    pub tick_rate: Duration, // TODO - make variable to support increased rate during shutdown.
    // TODO - cache?
    // TODO - ops?
    pub cycle_stats: Vec<Duration>,
    pub last_cycle_stats: Vec<Duration>,
    pub players: PlayerList,
    pub npcs: NPCList,
    pub new_players: Arc<Mutex<Vec<Player>>>,
    // TODO - game_map
    // TODO - zone_tracking
}

static mut ENGINE: Option<Engine> = None;
static INIT: Once = Once::new();

impl Engine {
    const MAX_PLAYERS: usize = 2048;
    const MAX_NPCS: usize = 8192;

    const TIMEOUT_NO_CONNECTION: i32 = 50;
    const TIMEOUT_NO_RESPONSE: i32 = 100;

    const AFK_EVENTRATE: i32 = 500;
    
    const INVALID_PID: usize = 5000;
    
    // We don't need safety, we're smart
    pub fn init() {
        INIT.call_once(|| {
            info!("Initializing global engine instance.");
            unsafe {
                ENGINE = Some(Engine::new());
            }
        })
    }

    pub fn get() -> &'static mut Engine {
        unsafe {
            match &mut ENGINE {
                Some(engine) => engine,
                None => {
                    // Auto-initialize if needed;
                    Self::init();
                    ENGINE.as_mut().unwrap()
                }
            }
        }
    }

    pub fn current_tick() -> i32 {
        Self::get().current_tick
    }

    pub fn new() -> Engine {
        Engine {
            members: false,
            current_tick: 0,
            tick_rate: Duration::from_millis(600),
            cycle_stats: vec![Duration::new(0, 0); 12],
            last_cycle_stats: vec![Duration::new(0, 0); 12],
            players: PlayerList::new(Engine::MAX_PLAYERS - 1),
            npcs: NPCList::new(Engine::MAX_NPCS - 1),
            new_players: Default::default(),
        }
    }

    pub fn start(&mut self, start_cycle: bool) {
        if let Err(e) = update_compiler() {
            error!("Failed to update compiler: {}", e);
        }

        revalidate_pack();
        generate_server_symbols();

        if let Err(e) = ensure_initialized() {
            error!("Failed to initialize cache: {}", e);
        } else {
            debug!("Cache successfully initialized.");
        }

        if let Err(e) = initialize_xtea() {
            error!("Failed to initialize XTEA module: {}", e);
        } else {
            debug!("XTEA module initialized.");
        }

        ScriptProvider::load();

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
                                    let mut game_client = GameClient::new(stream);

                                    loop {
                                        if game_client.state == ConnectionState::New && game_client.is_connection_active() {
                                            Self::on_new_connection(&mut game_client, thread_player.clone());
                                        } else {
                                            break
                                        }
                                    }
                                });
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
        let mut aggregate_duration = Duration::new(0, 0);
        let mut min_duration = Duration::new(u64::MAX, 0);
        let mut max_duration = Duration::new(0, 0);
        let mut tick_count = 0;
    
        let mut next_tick_time = Instant::now();

        loop {
            let start = Instant::now();
            
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

            let elapsed = start.elapsed();
            self.cycle_stats[engine_stat::CYCLE] = elapsed;
            std::mem::swap(&mut self.cycle_stats, &mut self.last_cycle_stats);

            aggregate_duration += elapsed;
            min_duration = min_duration.min(elapsed);
            max_duration = max_duration.max(elapsed);
            tick_count += 1;

            if self.current_tick % 10 == 9 {
                let avg_duration = if tick_count > 0 {
                    aggregate_duration / tick_count as u32
                } else {
                    Duration::new(0, 0)
                };

                info!(
                    "Ticks: {} to {} | Total: {:?} | Avg: {:?} | Min: {:?} | Max: {:?}",
                    self.current_tick - tick_count + 1,
                    self.current_tick + 1,
                    aggregate_duration,
                    avg_duration,
                    min_duration,
                    max_duration
                );

                aggregate_duration = Duration::new(0, 0);
                min_duration = Duration::new(u64::MAX, 0);
                max_duration = Duration::new(0, 0);
                tick_count = 0;
            }

            self.current_tick += 1;

            next_tick_time += self.tick_rate;
        
            let now = Instant::now();
            if next_tick_time > now {
                sleep(next_tick_time - now);
            } else {
                next_tick_time = now + self.tick_rate;
                if next_tick_time < now + self.tick_rate / 2 {
                    debug!("Server running behind schedule at tick {}", self.current_tick);
                }
            }
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
        self.npcs.for_each_mut(|npc| {
            // Check if npc is active
            if npc.active() {
                // Hunts will process even if the npc is delayed during this portion
                // TODO
            }
        });
        self.cycle_stats[engine_stat::WORLD] = start.elapsed();
    }

    /// - Calculate AFK event readiness
    /// - Process Packets
    /// - Process pathfinding/following requests
    /// - Client input tracking
    fn process_in(&mut self) {
        let start: Instant = Instant::now();
        
        // TODO - separate out stat?
        //self.cycle_stats[EngineStat::BandwidthIn as usize] = 0;

        self.players.for_each_mut(|player| {
            player.playtime += 1;

            if player.is_client_connected() && player.decode_in(self.current_tick) {

            }
        });

        // TODO - client input tracking

        // TODO - process pathfinding/following
        self.cycle_stats[engine_stat::CLIENTS_IN] = start.elapsed();
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
        self.cycle_stats[engine_stat::NPCS] = start.elapsed();
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

        self.cycle_stats[engine_stat::PLAYERS] = start.elapsed();
    }
    
    /// Player logouts
    fn process_logouts(&mut self) {
        let start: Instant = Instant::now();

        let mut pids_to_remove = Vec::with_capacity(16);
        self.players.for_each_mut(|player| {
            let mut force: bool = false;

            if (self.current_tick - player.initial_connect_tick) - player.last_response >= Self::TIMEOUT_NO_RESPONSE {
                // X-logged / timed out for 60s: force logout.
                debug!("X-logged on tick: {}, player last response: {}", self.current_tick, player.last_response);
                player.logging_out = true;
                force = true;
            } else if (self.current_tick - player.initial_connect_tick) - player.last_connected >= Self::TIMEOUT_NO_CONNECTION {
                // Connection lost for 30s: request idle logout.
                debug!("Idle log on tick: {}, player last connected: {}", self.current_tick, player.last_connected);
                player.request_idle_logout = true;
            }

            if player.request_logout || player.request_idle_logout {
                if self.current_tick >= player.prevent_logout_until {
                    player.logging_out = true;
                }
                player.request_logout = false;
                player.request_idle_logout = false;
            }

            if (player.logging_out) && (force || self.current_tick >= player.prevent_logout_until) {
                pids_to_remove.push(player.get_pid());
            }
        });

        // TODO

        for pid in pids_to_remove {
            self.remove_player(pid)
        }

        self.cycle_stats[engine_stat::LOGOUTS] = start.elapsed();
    }
    
    /// Player logins
    ///
    /// Before packets so they immediately load, but after processing so nothing hits them.
    fn process_logins(&mut self) {
        let start: Instant = Instant::now();

        {
            let shared_players = self.new_players.lock().unwrap();
            if shared_players.is_empty() {
                self.cycle_stats[engine_stat::LOGINS] = start.elapsed();
                return;
            }
        }

        let player_to_add = {
            let mut shared_players = self.new_players.lock().unwrap();
            shared_players.drain(..).collect::<Vec<Player>>()
        };

        for mut player in player_to_add {
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

            match self.get_next_pid(Some(&player.client)) {
                Ok(pid) => {
                    player.client.write_packet().expect("Failed to write packet to new connection");
                    player.set_pid(pid);
                    self.players.set(pid, player).expect("Failed to set player!");

                    if let Some(player_ref) = self.players.get_mut(pid) {
                        player_ref.on_login(self.current_tick);
                    }
                },
                Err(_err) => {
                    player.client.outbound = Packet::new(1);
                    player.client.outbound.p1(login_out::WORLD_FULL);
                    player.client.write_packet().expect("Failed to write packet to new connection");
                    player.client.shutdown();
                }
            };
        }

        self.cycle_stats[engine_stat::LOGINS] = start.elapsed();
    }
    
    /// Build list of active zones around players
    ///
    /// [loc] & [obj] despawn/respawn
    ///
    /// Compute shared buffer
    fn process_zones(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.cycle_stats[engine_stat::ZONES] = start.elapsed();
    }
    
    /// Convert player movements
    ///
    /// Compute player info
    ///
    /// Convert npc movements
    ///
    /// Compute npc info
    fn process_info(&mut self) {
        let start: Instant = Instant::now();
        // TODO - add benchmark value for this?
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
        self.players.for_each_mut(|player| {
            if !player.is_client_connected() {
                return;
            }
            // TODO
            player.encode_out();
        });
        self.cycle_stats[engine_stat::CLIENTS_OUT] = start.elapsed();
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
        self.cycle_stats[engine_stat::CLEANUP] = start.elapsed();
    }

    fn process_shutdown(&mut self) {
        self.players.for_each_mut(|player| {
            if player.is_client_connected() {
                player.logout();
                player.client.shutdown();
            }
        });
    }

    #[inline]
    pub fn remove_player(&mut self, pid: usize) {
        if let Some(player_ref) = self.players.get_mut(pid) {
            if player_ref.is_client_connected() {
                player_ref.client.shutdown();
            }
            player_ref.set_active(false);
        }
        self.players.remove(pid);
    }

    fn on_new_connection(client: &mut GameClient, thread_player: Arc<Mutex<Vec<Player>>>) {
        
        if let Err(err) = client.read_packet_with_size(1) {
            error!("Failed to read packet from client: {}", err);
            client.shutdown();
            return
        }

        client.opcode = client.inbound().g1();
        if client.opcode == title_protocol::INIT_GAME_CONNECTION {
            client.read_packet_with_size(1).unwrap();

            // Used to load-balance.
            let _username_hash = client.inbound().g1();
            client.outbound.p1(0);

            // Server session key for this connection, used in decrypting return values.
            let session_key: u64 = ((rand::random::<f64>() * 99999999.0) as u64) << 32 | ((rand::random::<f64>() * 99999999.0) as u64);
            client.outbound.p8(session_key as i64);
            client.write_packet().expect("Failed to write packet to new connection");
        }  else if client.opcode == title_protocol::RECONNECT || client.opcode == title_protocol::LOGIN {
            // RECONNECT & LOGIN packet length is variable, length indicated by 'short' after opcode.
            client.read_packet_with_size(2).unwrap();
            let payload_length = client.inbound.g2();
            client.read_packet_with_size(payload_length as usize).unwrap();

            let client_revision = client.inbound.g4();
            if client_revision != 530 {
                client.outbound.p1(login_out::CLIENT_OUT_OF_DATE);
                client.write_packet().expect("Failed to write packet to new connection");
                client.shutdown();
                return
            }

            // Data here is unknown. 
            // Populated through client script opcode [5600]. 
            let bytes1 = client.inbound().g1b();
            
            let adverts_suppressed = client.inbound().g1b();
            let client_signed = client.inbound().g1b();
            
            // Window status block
            let window_mode = window_mode::from_i8(client.inbound.g1b());
            let canvas_width = client.inbound().g2()  as u32;
            let canvas_height = client.inbound().g2()  as u32;
            let anti_aliasing_mode = client.inbound().g1b() as u32;
            let window_status: WindowStatus = WindowStatus::new(window_mode, canvas_width, canvas_height, anti_aliasing_mode);
            
            let uuid = client.inbound().gbytes(24);
            let site_settings_cookie = client.inbound().gjstr();
            let affiliate_id = client.inbound().g4();
            let detail_options = client.inbound().g4();
            let verify_id = client.inbound().g2();
            
            for i in 0..28 {
                let checksum = client.inbound().g4() as u32;
                if checksum != get_checksum(i).expect("Failed to get checksum for archive") {
                    client.outbound.p1(login_out::CLIENT_OUT_OF_DATE);
                    client.write_packet().expect("Failed to write packet to new connection");
                    client.shutdown();
                    return
                }
            }

            let rsa_block_length = client.inbound().g1();
            let mut rsa_packet_decrypted = rsa::decrypt_rsa_block(client.inbound.clone(), rsa_block_length as usize);

            let rsa_verification = rsa_packet_decrypted.g1();
            if rsa_verification != 10 {
                debug!("RSA verification failed, received value: {}", rsa_verification);
                client.outbound.p1(login_out::INVALID_LOGIN_PACKET);
                client.write_packet().expect("Failed to write packet to new connection");
                client.shutdown();
                return
            }

            // Sent on login, however it has no function in revision 530.
            let _xtea_key = XTEAKey(
                rsa_packet_decrypted.g4() + 50,
                rsa_packet_decrypted.g4() + 50,
                rsa_packet_decrypted.g4() + 50, 
                rsa_packet_decrypted.g4() + 50
            );
            
            let username = decode37(rsa_packet_decrypted.g8());
            let password = rsa_packet_decrypted.gjstr();

            if client.opcode == title_protocol::RECONNECT {
                client.outbound.p1(login_out::RECONNECT_OK);
            } else if client.opcode == title_protocol::LOGIN {
                client.outbound.p1(login_out::OK);
            }

            client.opcode = 0;
            client.state = ConnectionState::Connected;

            let mut new_client = Some(std::mem::replace(
                client,
                GameClient::new_dummy()
            ));
            
            let player = Player::new(
                &mut new_client, 
                CoordGrid::from(3200, 0, 3200), 
                0, 
                window_status, 
                0, 
                Self::INVALID_PID,
                verify_id,
                username
            );

            let mut players_lock = thread_player.lock().unwrap();
            players_lock.push(player);
        } else {
            debug!("Invalid opcode from initial connection: [{}]", client.opcode);
            client.outbound.p1(login_out::INVALID_LOGIN_PACKET);
            client.write_packet().expect("Failed to write packet to new connection");
            client.shutdown();
            return
        }
    }

    fn get_next_pid(&self, client: Option<&GameClient>) -> Result<usize, &'static str>  {
        let client = match client {
            Some(c) => c,
            None => return self.players.next(false, None),
        };
        
        let default = || self.players.next(false, None);
        let stream = match &client.stream {
            Some(s) => s,
            None => return default(),
        };

        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(_) => return default(),
        };

        match peer_addr.ip() {
            IpAddr::V4(ipv4) => {
                let last_octet = ipv4.octets()[3];
                let start = ((last_octet % 20) * 100) as usize;
                self.players.next(true, Some(start))
            },
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                let third_segment = segments[2];
                let start = ((third_segment % 20) * 100) as usize;
                self.players.next(true, Some(start))
            }
        }
    }
}