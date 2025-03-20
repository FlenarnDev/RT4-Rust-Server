#[derive(Debug)]
pub struct If_OpenTop {
    pub(crate) id: u32,
    pub(crate) interface_type: u8,
    pub(crate) verify_id: u32,
}

pub const DEFAULT: u8 = 0;
pub const RESET: u8 = 2;

impl If_OpenTop {
    pub fn new(id: u32, interface_type: bool, verify_id: u32) -> If_OpenTop {
        If_OpenTop {
            id, 
            interface_type: if interface_type { RESET } else { DEFAULT },
            verify_id
        }
    }
}