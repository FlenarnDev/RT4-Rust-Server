#[derive(Debug)]
pub struct InfoMessage {}

impl InfoMessage {
    pub fn new() -> Self {
        InfoMessage {}
    }
    
    fn persists(&self) -> bool {
        false
    }
}