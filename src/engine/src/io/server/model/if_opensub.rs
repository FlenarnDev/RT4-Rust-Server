#[derive(Debug, Clone, PartialEq)]
pub struct If_OpenSub {
    pub(crate) window_id: u32,
    pub(crate) component_id: u32,
    pub(crate) interface_id: u32,
    pub(crate) flags: u32,
    pub(crate) verify_id: u32,
}

impl If_OpenSub {
    pub fn new(window_id: u32, component_id: u32, interface_id: u32, flags: u32, verify_id: u32) -> If_OpenSub {
        If_OpenSub {
            window_id,
            component_id,
            interface_id,
            flags,
            verify_id,
        }
    }
}