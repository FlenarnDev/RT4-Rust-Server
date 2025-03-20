#[derive(Debug)]
pub struct If_OpenSub {
    pub(crate) flags: u32,
    pub(crate) verify_id: u32,
    pub(crate) window_id: u32,
    pub(crate) interface_id: u32,
}

impl If_OpenSub {
    pub fn new(verify_id: u32, window_id: u32, interface_id: u32,flags: u32, ) -> If_OpenSub {
        If_OpenSub {
            flags, 
            verify_id,
            window_id,
            interface_id
        }
    }
}