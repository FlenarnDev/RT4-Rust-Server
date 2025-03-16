use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct ClientProtocol {
    pub id: u32,
    pub length: i32
}

impl ClientProtocol {
    pub const NO_TIMEOUT: Self = Self { 
        id: 93, 
        length: 0 
    };
    
    pub const WINDOW_STATUS: Self = Self {
        id: 243,
        length: 6
    };
}

lazy_static! {
    pub static ref BY_ID: Vec<Option<ClientProtocol>> = {
        let mut v = vec![None; 256];
        v[ClientProtocol::NO_TIMEOUT.id as usize] = Some(ClientProtocol::NO_TIMEOUT.clone());
        v[ClientProtocol::WINDOW_STATUS.id as usize] = Some(ClientProtocol::WINDOW_STATUS.clone());
        v
    };
}