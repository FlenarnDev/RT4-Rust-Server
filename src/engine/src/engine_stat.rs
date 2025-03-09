#[repr(u8)]
pub enum EngineStat {
    Cycle,
    World,
    ClientsIn,
    Npcs,
    Players,
    Logouts,
    Logins,
    Zones,
    ClientsOut,
    Cleanup,
    BandwidthIn,
    BandwidthOut,
}