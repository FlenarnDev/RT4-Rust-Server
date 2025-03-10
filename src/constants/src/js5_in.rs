pub mod js5_in {
    /// Requests a group. Prefetch requests are used to prepopulate most of the cache in the background.
    pub const PREFETCH: u8 = 0;
    
    /// Requests a group. Urgent requests have a higher priority than prefetch requests, as the client needs the group immediately.
    pub const URGENT: u8 = 1;
    
    /// Sent whenever the player logs into the game.
    /// Consensus in the community is that the logged in/out state is probably
    /// used for prioritisation, much like the distinction between prefetch/urgent.
    pub const LOGGED_IN: u8 = 2;
    
    /// Sent whenever the player logs out of the game.
    pub const LOGGED_OUT: u8 = 3;
   
    /// Sent to set the encryption key.
    pub const REKEY: u8 = 4;
    
    /// Sent immediately after the JS5 connection is established. Its purpose is not known.
    pub const CONNECTED: u8 = 6;
    
    /// Requests that the server closes the connection. Sent by the `::serverjs5drop` command.
    pub const DISCONNECT: u8 = 7;
}