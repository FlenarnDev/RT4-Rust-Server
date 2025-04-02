use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::model::event_applet_focus::EventAppletFocus;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

impl MessageDecoder<EventAppletFocus> for EventAppletFocus {
    fn length() -> i32 {
        1
    }

    fn decode(_: ClientProtocol, mut packet: Packet) -> EventAppletFocus {
        EventAppletFocus::new(
            packet.g1() == 1
        )
    }
}
