#[derive(Debug, Clone, Copy)]
pub struct ServerProtocol {
    pub id: i32,
    pub length: i32,
}

impl ServerProtocol {
    pub const fn new(id: i32, length: i32) -> ServerProtocol {
        ServerProtocol { id, length }
    }
    
    pub const REBUILD_NORMAL: ServerProtocol = ServerProtocol::new(162, -2);
}