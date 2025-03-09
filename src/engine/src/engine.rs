use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use log::{debug, info};
use crate::engine_stat::EngineStat;
use crate::entity::npc::Npc;
use crate::entity::player::Player;
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
        }
    }
    
    // TODO - mock function?
    
    pub fn start(&mut self, start_cycle: bool) {
        info!("Starting world...");
        
        let mut player: Player = Player::new(CoordGrid::from(3094, 0, 3106), 0);
        player.uid = 0;
        self.add_player(player);
        
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
                "Tick: {} took: {:?}",
                self.current_tick.current_tick,
                self.cycle_stats[EngineStat::Cycle as usize]
            );
            
            // Cycle the world now
            self.current_tick.increment();
            
            sleep(self.tick_rate.saturating_sub(start.elapsed()));
        }
    }
    
    /// - World Queue
    /// - Calculate AFK event readiness
    /// - NPC Spawn script
    /// - NPC Hunt
    fn process_world(&mut self) {
        let start: Instant = Instant::now();
        // TODO
        self.cycle_stats[EngineStat::World as usize] = start.elapsed();
    }
    
    /// - Decode Packets
    /// - Process pathfinding/following
    fn process_in(&mut self) {
        let start: Instant = Instant::now();
        
        // TODO - separate out stat?
        //self.cycle_stats[EngineStat::BandwidthIn as usize] = 0;

        for (_, player) in &self.players {
            let _: Ref<Player> = player.borrow();
            player.borrow_mut().playtime += 1;
            debug!("Player playtime: {}", player.borrow().playtime);
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
        // TODO
        self.cycle_stats[EngineStat::Logins as usize] = start.elapsed();
    }
    
    /// Build list of active zones around players
    /// loc & obj despawn/respawn
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
}