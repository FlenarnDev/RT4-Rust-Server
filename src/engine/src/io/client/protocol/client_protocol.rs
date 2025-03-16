use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct ClientProtocol {
    pub id: u32,
    pub length: i32
}

impl ClientProtocol {
    
    pub const EVENT_CAMERA_POSITION: Self = ClientProtocol { id: 21, length: 4 };
    pub const MAP_REBUILD_COMPLETE: Self = ClientProtocol { id: 110, length: 0 };
    pub const NO_TIMEOUT: Self = ClientProtocol { id: 93, length: 0 };
    pub const VERIFICATION: Self = ClientProtocol { id: 20, length: 4 };
    pub const WINDOW_STATUS: Self = ClientProtocol { id: 243, length: 6 };
    pub const TRANSMITVAR_VERIFYID: Self = ClientProtocol { id: 177, length: 2 };
}

lazy_static! {
    pub static ref BY_ID: Vec<Option<ClientProtocol>> = {
        let mut v = vec![None; 256];
        v[ClientProtocol::EVENT_CAMERA_POSITION.id as usize] = Some(ClientProtocol::EVENT_CAMERA_POSITION);
        v[ClientProtocol::MAP_REBUILD_COMPLETE.id as usize] = Some(ClientProtocol::MAP_REBUILD_COMPLETE);
        v[ClientProtocol::NO_TIMEOUT.id as usize] = Some(ClientProtocol::NO_TIMEOUT);
        v[ClientProtocol::VERIFICATION.id as usize] = Some(ClientProtocol::VERIFICATION);
        v[ClientProtocol::WINDOW_STATUS.id as usize] = Some(ClientProtocol::WINDOW_STATUS);
        v[ClientProtocol::TRANSMITVAR_VERIFYID.id as usize] = Some(ClientProtocol::TRANSMITVAR_VERIFYID);
        v
    };
}