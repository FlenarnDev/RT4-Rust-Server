use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct ClientProtocol {
    pub id: u32,
    pub length: i32
}

impl ClientProtocol {
    pub const EVENT_APPLET_FOCUS: Self = ClientProtocol { id: 22, length: 1 };
    pub const EVENT_CAMERA_POSITION: Self = ClientProtocol { id: 21, length: 4 };
    pub const EVENT_MOUSE_CLICK: Self = ClientProtocol { id: 75, length: 6 };
    pub const MAP_REBUILD_COMPLETE: Self = ClientProtocol { id: 110, length: 0 };
    pub const NO_TIMEOUT: Self = ClientProtocol { id: 93, length: 0 };
    pub const VERIFICATION: Self = ClientProtocol { id: 20, length: 4 };
    pub const WINDOW_STATUS: Self = ClientProtocol { id: 243, length: 6 };
    pub const TRANSMITVAR_VERIFYID: Self = ClientProtocol { id: 177, length: 2 };
}

lazy_static! {
    pub static ref BY_ID: Vec<Option<ClientProtocol>> = {
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
        
        let mut v = vec![None; 256];
        protocols.iter().for_each(|protocol| {
            v[protocol.id as usize] = Some(protocol.clone());
        });
        v
    };
}