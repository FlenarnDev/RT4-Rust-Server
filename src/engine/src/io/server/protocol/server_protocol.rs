#[derive(Debug, Clone, Copy)]
pub struct ServerProtocol {
    pub id: i32,
    pub length: i32,
}

impl ServerProtocol {
    pub const fn new(id: i32, length: i32) -> ServerProtocol {
        ServerProtocol { id, length }
    }

    // Map
    pub const REBUILD_NORMAL: ServerProtocol = ServerProtocol::new(162, -2);

    // Update
    pub const NPC_INFO: ServerProtocol = ServerProtocol::new(32, -2);
    pub const PLAYER_INFO: ServerProtocol = ServerProtocol::new(225, -2);

    // var{p, c, bit}
    pub const CLIENT_SETVARC_SMALL: ServerProtocol = ServerProtocol::new(65, 5);
    pub const CLIENT_SETVARC_LARGE: ServerProtocol = ServerProtocol::new(69, 8);
    
    // Interfaces
    pub const IF_OPENSUB: ServerProtocol = ServerProtocol::new(145, 5);
    
    // Misc.
    pub const LOGOUT: ServerProtocol = ServerProtocol::new(86, 0);
    pub const UPDATE_RUNENERGY: ServerProtocol = ServerProtocol::new(234, 1);
    pub const UPDATE_RUNWEIGHT: ServerProtocol = ServerProtocol::new(159, 2);
    pub const UPDATE_REBOOT_TIME: ServerProtocol = ServerProtocol::new(85, 2);
    pub const MIDI_SONG: ServerProtocol = ServerProtocol::new(4, 2);
    pub const MIDI_JINGLE: ServerProtocol = ServerProtocol::new(208, 5);
    pub const SYNTH_SOUND: ServerProtocol = ServerProtocol::new(172, 5);
}