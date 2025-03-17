use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::event_applet_focus::EventAppletFocusMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

pub struct EventAppletFocusDecoder;

impl MessageDecoder for EventAppletFocusDecoder {
    type Message = EventAppletFocusMessage;

    fn protocol(&self) -> &ClientProtocol {
        &ClientProtocol::EVENT_APPLET_FOCUS
    }

    fn decode(&self, packet: &mut Packet, _length: usize) -> Box<Self::Message> {
        let focus: bool = packet.g1() == 1;
        Box::new(EventAppletFocusMessage{focus})
    }
}