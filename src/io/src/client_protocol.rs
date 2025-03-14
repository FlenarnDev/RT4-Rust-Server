use lazy_static::lazy_static;


#[derive(Debug, Clone)]
pub struct ClientProtocol {
    pub id: i32,
    pub length: i32
}

impl ClientProtocol {
    pub const NO_TIMEOUT: Self = Self { 
        id: 93, 
        length: 0 
    };
}

lazy_static! {
    pub static ref BY_ID: Vec<Option<ClientProtocol>> = {
        let mut v = vec![None; 256];
        v[ClientProtocol::NO_TIMEOUT.id as usize] = Some(ClientProtocol::NO_TIMEOUT.clone());
        v
    };
    
}