use crate::io::packet::Packet;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::model::message_game::Message_Game;
use crate::io::server::protocol::server_protocol::ServerProtocol;

pub struct Message_Game_Encoder;

impl Message_Game_Encoder {
    #[inline]
    pub fn new() -> Self { Message_Game_Encoder }
}

impl MessageEncoder<Message_Game> for Message_Game_Encoder {
    #[inline]
    fn protocol(&self) -> ServerProtocol { ServerProtocol::MESSAGE_GAME }

    fn encode(&self, packet: &mut Packet, message: Message_Game) {
        let mut temporary_packet: Packet = Packet::new(50);
        temporary_packet.pjstr(message.message.as_str(), 0);
        packet.p1(temporary_packet.data.len() as i32);
        packet.pbytes(&*temporary_packet.data, 0, temporary_packet.data.len());
    }
}