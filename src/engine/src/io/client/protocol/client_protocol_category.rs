#[repr(u8)]
// TODO: Measure how many events we should expect to receive from the client.
// OSRS has this as 50/10 but we know that's not true in `RS2`.
// TODO: Determine which packets belong in which category for this era.
pub enum ClientProtocolCategory {
    CLIENT_EVENT = 0,
    USER_EVENT = 1,
    RESTRICTED_EVENT = 2, // Flood restricted events
}

#[repr(u8)]
/// Packet decoding limit per tick, exceeding this ends decoding and picks up where it left off on the next tick.
pub enum ClientProtocolCategoryLimit {
    CLIENT_EVENT = 20,
    USER_EVENT = 5,
    RESTRICTED_EVENT = 2,
}