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
        packet.p1(message.message.len() as i32 + 1);
        packet.pjstr(&message.message, 0);
    }
}