use crate::io::packet::Packet;

pub struct PacketPool {
    packet_100b: Packet,
    packet_5kb: Packet,
}

impl PacketPool {
    pub fn new() -> PacketPool {
        PacketPool {
            packet_100b: Packet::new(100),
            packet_5kb: Packet::new(5000),
        }
    }
    
    pub fn take(&mut self, length: usize) -> &mut Packet {
        if length <= 100 {
            let packet: &mut Packet = &mut self.packet_100b;
            packet.position = 0;
            packet.bit_position = 0;
            packet
        } else {
            let packet: &mut Packet = &mut self.packet_5kb;
            packet.position = 0;
            packet.bit_position = 0;
            packet
        }
    }
}