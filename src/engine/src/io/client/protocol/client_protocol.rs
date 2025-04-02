use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(TryFromPrimitive)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum ClientProtocol {
    EVENT_APPLET_FOCUS,
    EVENT_CAMERA_POSITION,
    EVENT_MOUSE_CLICK,
    
    MAP_REBUILD_COMPLETE,
    NO_TIMEOUT,
    VERIFICATION,
    WIDNOW_STATUS,
    TRANSMITVAR_VERIFYID,
}

#[repr(u8)]
pub enum ClientInternalProtocol {
    EVENT_CAMERA_POSITION = 21,
    EVENT_APPLET_FOCUS = 22,
    EVENT_MOUSE_CLICK = 75,
    
    MAP_REBUILD_COMPLETE = 110,
    
    NO_TIMEOUT = 93,
    VERIFICATION = 20,
    
    WINDOW_STATUS = 243,
    
    TRANSMITVAR_VERIFYID = 177,
}