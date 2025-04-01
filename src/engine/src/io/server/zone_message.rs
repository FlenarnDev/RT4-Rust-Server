#[derive(Debug, Clone, PartialEq)]
pub struct ZoneMessage {}

impl ZoneMessage {
    pub fn new() -> Self {
        ZoneMessage {}
    }
    
    fn persists(&self) -> bool {
        false
    }
}