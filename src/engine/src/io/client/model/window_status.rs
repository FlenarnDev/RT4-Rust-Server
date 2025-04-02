use constants::window_mode::window_mode;

#[derive(Debug)]
pub struct WindowStatus {
    pub(crate) window_mode: window_mode,
    pub(crate) canvas_width: u32,
    pub(crate) canvas_height: u32,
    pub(crate) anti_aliasing_mode: u32
}

impl WindowStatus {
    pub fn new(window_mode: window_mode, canvas_width: u32, canvas_height: u32, anti_aliasing_mode: u32) -> WindowStatus {
        WindowStatus {
            window_mode,
            canvas_width,
            canvas_height,
            anti_aliasing_mode
        }
    }
}