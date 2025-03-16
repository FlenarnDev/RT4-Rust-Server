/// Packet decoding limit per tick, exceeding this ends decoding and picks up where it left off on the next tick.
pub struct ClientProtocolCategory {
    pub id: u8,
    pub limit: u8,
}

impl ClientProtocolCategory {
    pub const fn new(id: u8, limit: u8) -> ClientProtocolCategory {
        Self { id, limit }
    }
    
    /// TODO - measure how many events we should expect to receive from the client.
    ///
    /// OSRS has this as [50, 10], but we know this isn't true in RS2.
    /// 
    /// TODO - determine which packets belong in which category.
    pub const CLIENT_EVENT: ClientProtocolCategory = ClientProtocolCategory::new(0, 20);
    pub const USER_EVENT: ClientProtocolCategory = ClientProtocolCategory::new(1, 5);
    /// Flood restricted events.
    pub const RESTRICTED_EVENT: ClientProtocolCategory = ClientProtocolCategory::new(2, 2);
}