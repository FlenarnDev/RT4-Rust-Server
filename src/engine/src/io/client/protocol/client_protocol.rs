use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProtocolId(pub u32);

#[derive(Debug, Clone)]
pub struct ClientProtocol {
    pub id: ProtocolId,
    pub length: i32,
}

impl ClientProtocol {
    pub const EVENT_APPLET_FOCUS: Self = ClientProtocol { id: ProtocolId(22), length: 1 };
    pub const EVENT_CAMERA_POSITION: Self = ClientProtocol { id: ProtocolId(21), length: 4 };
    pub const EVENT_MOUSE_CLICK: Self = ClientProtocol { id: ProtocolId(75), length: 6 };
    pub const MAP_REBUILD_COMPLETE: Self = ClientProtocol { id: ProtocolId(110), length: 0 };
    pub const NO_TIMEOUT: Self = ClientProtocol { id: ProtocolId(93), length: 0 };
    pub const VERIFICATION: Self = ClientProtocol { id: ProtocolId(20), length: 4 };
    pub const WINDOW_STATUS: Self = ClientProtocol { id: ProtocolId(243), length: 6 };
    pub const TRANSMITVAR_VERIFYID: Self = ClientProtocol { id: ProtocolId(177), length: 2 };
}

lazy_static! {
    pub static ref PROTOCOL_MAP: HashMap<ProtocolId, ClientProtocol> = {
        let protocols = [
            ClientProtocol::EVENT_APPLET_FOCUS,
            ClientProtocol::EVENT_CAMERA_POSITION,
            ClientProtocol::EVENT_MOUSE_CLICK,
            ClientProtocol::MAP_REBUILD_COMPLETE,
            ClientProtocol::NO_TIMEOUT,
            ClientProtocol::VERIFICATION,
            ClientProtocol::WINDOW_STATUS,
            ClientProtocol::TRANSMITVAR_VERIFYID,
        ];
        
        let mut map = HashMap::new();
        protocols.iter().for_each(|protocol| {
            map.insert(protocol.id, protocol.clone());
        });
        map
    };
}

// For backward compatibility with existing code
pub fn get_protocol_by_id(id: u32) -> Option<&'static ClientProtocol> {
    PROTOCOL_MAP.get(&ProtocolId(id))
}