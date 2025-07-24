use crate::io::packet::Packet;

pub trait ConfigType {
    fn id(&self) -> u32;
    
    fn debugname(&self) -> Option<&String>;
    fn set_debugname(&mut self, debugname: String);
    
    fn decode(&mut self, opcode: u8, packet: &mut Packet);
    
    fn decode_type(&mut self, packet: &mut Packet, opcode_order: &mut Vec<u8>) {
        while packet.remaining() > 0 {
            let opcode = packet.g1();
            
            opcode_order.push(opcode);
            
            if opcode == 0 {
                break;
            }
            self.decode(opcode, packet);
        }
    }
}