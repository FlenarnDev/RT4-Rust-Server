pub mod engine_stat {
    pub const CYCLE: usize = 0;
    pub const WORLD: usize = 1;
    pub const CLIENTS_IN: usize = 2;
    pub const NPCS: usize = 3;
    pub const PLAYERS: usize = 4;
    pub const LOGOUTS: usize = 5;
    pub const LOGINS: usize = 6;
    pub const ZONES: usize = 7;
    pub const CLIENTS_OUT: usize = 8;
    pub const CLEANUP: usize = 9;
    pub const BANDWIDTH_IN: usize = 10;
    pub const BANDWIDTH_OUT: usize = 11;

    // Optionally, include the count for array initialization
    pub const COUNT: usize = 12;
}