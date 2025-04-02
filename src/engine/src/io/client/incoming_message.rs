#[derive(Clone)]
pub struct IncomingMessage {
    pub id: i32,
    pub length: i32,
}

impl IncomingMessage {
    #[inline]
    pub fn new(id: i32, length: i32) -> IncomingMessage {
        IncomingMessage {
            id,
            length,
        }    
    }
}