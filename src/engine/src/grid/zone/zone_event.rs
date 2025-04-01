use crate::grid::zone::zone_type_event::ZoneEventType;
use crate::io::server::zone_message::ZoneMessage;

pub struct ZoneEvent {
    pub zone_event_type: ZoneEventType,
    pub receiver: f64,
    pub message: ZoneMessage,
}

impl ZoneEvent {
    pub fn new(zone_event_type: ZoneEventType, receiver: f64, message: ZoneMessage) -> ZoneEvent {
        ZoneEvent {
            zone_event_type,
            receiver,
            message,
        }
    }
}