use crate::io::packet::Packet;

pub trait ConfigType {
    fn id(&self) -> u32;
    
    fn debugname(&self) -> Option<&String>;
    fn set_debugname(&mut self, debugname: String);
    
    fn decode(&mut self, opcode: u8, dat: &mut Packet);
    
    fn decode_type(&mut self, data: &mut Packet) {
        while data.remaining() > 0 {
            let opcode = data.g1();
            if opcode == 0 {
                break;
            }
            
            self.decode(opcode, data);
        }
    }
}