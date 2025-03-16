#[repr(i8)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum window_mode {
    NULL = -1,
    SD = 0,
    HD_NON_RESIZEABLE = 1,
    HD_RESIZEABLE = 2,
    HD_FULLSCREEN = 3
}

impl window_mode {
    pub fn from_i8(value: i8) -> window_mode {
        match value {
            0 => window_mode::SD,
            1 => window_mode::HD_NON_RESIZEABLE,
            2 => window_mode::HD_RESIZEABLE,
            3 => window_mode::HD_FULLSCREEN,
            _ => window_mode::NULL
        }
    }
}