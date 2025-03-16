use log::debug;
use cache::xtea::get_xtea_key_by_mapsquare;
use crate::io::packet::Packet;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::protocol::server_protocol::ServerProtocol;

pub struct RebuildNormalEncoder;

impl RebuildNormalEncoder {
    pub fn new() -> Self {
        RebuildNormalEncoder
    }
}

impl MessageEncoder<RebuildNormal> for RebuildNormalEncoder {
    fn protocol(&self) -> ServerProtocol {
        ServerProtocol::REBUILD_NORMAL
    }

    fn encode(&self, packet: &mut Packet, message: &RebuildNormal) {
        let mut temporary_packet: Packet = Packet::from(vec![]);
        temporary_packet.p2add(message.local_x());

        for mapsquare in message.mapsquares() {
            let xtea_key = get_xtea_key_by_mapsquare(mapsquare);
            if xtea_key.is_zero() {
                for _i in 0..4 {
                    temporary_packet.p4me(0);
                }
            } else {
                temporary_packet.p4me(xtea_key.0);
                temporary_packet.p4me(xtea_key.1);
                temporary_packet.p4me(xtea_key.2);
                temporary_packet.p4me(xtea_key.3);
            }
        }

        temporary_packet.p1(128);
        temporary_packet.p2(message.zone_x());
        temporary_packet.p2add(message.zone_z());
        temporary_packet.p2add(message.local_z());
        packet.p2(temporary_packet.data.len() as i32);
        packet.pbytes(&*temporary_packet.data, 0, temporary_packet.data.len());
    }
}