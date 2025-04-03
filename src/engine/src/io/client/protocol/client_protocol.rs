
#[repr(u8)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum ClientProtocol {
    EVENT_APPLET_FOCUS,
    EVENT_CAMERA_POSITION,
    EVENT_MOUSE_CLICK,
    
    MAP_REBUILD_COMPLETE,
    NO_TIMEOUT,
    VERIFICTATION,
    WIDNOW_STATUS,
    TRANSMITVAR_VERIFYID,
}

/*impl ClientProtocol {
    pub const EVENT_APPLET_FOCUS: Self = ClientProtocol { id: 22, length: 1 };
    pub const EVENT_CAMERA_POSITION: Self = ClientProtocol { id: 21, length: 4 };
    pub const EVENT_MOUSE_CLICK: Self = ClientProtocol { id: 75, length: 6 };
    pub const MAP_REBUILD_COMPLETE: Self = ClientProtocol { id: 110, length: 0 };
    pub const NO_TIMEOUT: Self = ClientProtocol { id: 93, length: 0 };
    pub const VERIFICATION: Self = ClientProtocol { id: 20, length: 4 };
    pub const WINDOW_STATUS: Self = ClientProtocol { id: 243, length: 6 };
    pub const TRANSMITVAR_VERIFYID: Self = ClientProtocol { id: 177, length: 2 };
}*/