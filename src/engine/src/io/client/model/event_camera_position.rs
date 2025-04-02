pub struct EventCameraPosition {
    pub(crate) camera_pitch: u16,
    pub(crate) camera_yaw: u16,
}

impl EventCameraPosition {
    #[inline]
    pub fn new(camera_pitch: u16, camera_yaw: u16) -> Self {
        EventCameraPosition {
            camera_pitch,
            camera_yaw
        }
    }
}