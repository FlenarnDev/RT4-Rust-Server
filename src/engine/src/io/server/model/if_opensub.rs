#[derive(Debug)]
pub struct If_OpenTop {
    pub(crate) component: u32,
    pub(crate) reset_worldmap: u8,
    pub(crate) verify_id: u32,
}

pub const DEFAULT: u8 = 0;
pub const RESET_WORLDMAP: u8 = 2;

impl If_OpenTop {
    pub fn new(component: u32, reset_worldmap: bool, verify_id: u32) -> If_OpenTop {
        If_OpenTop { 
            component, 
            reset_worldmap: if reset_worldmap { RESET_WORLDMAP } else { DEFAULT },
            verify_id
        }
    }
}