use constants::window_mode::window_mode;

#[derive(Copy, Clone)]
pub struct WindowStatus {
    pub window_mode: window_mode,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub anti_aliasing_mode: u32,
}

impl WindowStatus {
    pub fn new(window_mode: window_mode, canvas_width: u32, canvas_height: u32, anti_aliasing_mode: u32 ) -> Self {
        WindowStatus { window_mode, canvas_width, canvas_height, anti_aliasing_mode }
    }
}