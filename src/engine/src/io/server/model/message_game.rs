#[derive(Debug, Clone, PartialEq)]
pub struct Message_Game {
    pub(crate) message: String,
}

impl Message_Game {
    pub fn new(message: String) -> Message_Game {
        Message_Game {
            message
        }
    }
}