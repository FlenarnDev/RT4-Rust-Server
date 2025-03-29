use std::collections::HashMap;
use crate::io::packet::Packet;

#[derive(Debug)]
pub enum ParamValue {
    String(String),
    Integer(i32),
}

pub type Params = HashMap<i32, ParamValue>;

pub fn decode_params(packet: &mut Packet) -> Params {
    let count = packet.g1();
    // TODO - bitCeil handling for capacity
    let mut params: Params = HashMap::with_capacity(count as usize);
    
    for _ in 0..count {
        let is_string = packet.g1() == 1;
        let key = packet.g3();
        
        let value = if is_string {
            ParamValue::String(packet.gjstr())
        } else {
            ParamValue::Integer(packet.g4())
        };
        
        params.insert(key, value);
    }
    
    params
}