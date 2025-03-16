use crate::io::server::protocol::server_protocol::ServerProtocol;

#[derive(Debug, Clone, Copy)]
pub struct InfoProtocol {
    protocol: ServerProtocol,
}

impl InfoProtocol {
    pub fn new(id: i32, length: i32) -> InfoProtocol {
        InfoProtocol {
            protocol: ServerProtocol::new(id, length),            
        }
    }
    
    // CONSTS START HERE
    
    pub const fn id(&self) -> i32 {
        self.protocol.id
    }
    
    pub const fn length(&self) -> i32 {
        self.protocol.length
    }
}