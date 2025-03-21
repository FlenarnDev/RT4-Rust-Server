use std::collections::HashMap;
use std::path::Path;
use log::{debug, error};
use log::Level::Debug;
use sha2::digest::typenum::op;
use crate::io::packet::Packet;
use crate::script::script_opcode::ScriptOpcode;

#[derive(Debug, Clone)]
pub struct ScriptInfo {
    pub(crate) script_name: String,
    source_file_path: String,
    pub(crate) lookup_key: i32,
    parameter_types: Vec<i32>,
    pcs: Vec<i32>,
    lines: Vec<i32>,
}

pub type SwitchTable = HashMap<i32, Option<i32>>;

pub fn is_large_operand(opcode: i32) -> bool {
    if opcode > 100 {
        return false;
    }

    match opcode {
        x if x == ScriptOpcode::RETURN as i32 => false,
        x if x == ScriptOpcode::POP_INT_DISCARD as i32 => false,
        x if x == ScriptOpcode::POP_STRING_DISCARD as i32 => false,
        x if x == ScriptOpcode::GOSUB as i32 => false,
        x if x == ScriptOpcode::JUMP as i32 => false,
        _ => true
    }
}

#[derive(Debug, Clone)]
pub struct ScriptFile {
    pub info: ScriptInfo,
    pub id: i32,
    pub int_local_count: i32,
    pub string_local_count: i32,
    pub int_arg_count: i32,
    pub string_arg_count: i32,
    pub switch_tables: Vec<SwitchTable>,
    pub opcodes: Vec<ScriptOpcode>,
    pub int_operands: Vec<i32>,
    pub string_operands: Vec<String>,
}

impl Default for ScriptFile {
    fn default() -> Self {
        ScriptFile {
            info: ScriptInfo {
                script_name: String::from("<unknown>"),
                source_file_path: String::from("<unknown>"),
                lookup_key: -1,
                parameter_types: Vec::new(),
                pcs: Vec::new(),
                lines: Vec::new(),
            },
            id: -1,
            int_local_count: 0,
            string_local_count: 0,
            int_arg_count: 0,
            string_arg_count: 0,
            switch_tables: Vec::new(),
            opcodes: Vec::new(),
            int_operands: Vec::new(),
            string_operands: Vec::new(),
        }
    }
}

impl ScriptFile {
    pub fn new(id: i32) -> Self {
        let mut script = ScriptFile::default();
        script.id = id;
        script
    }
    
    pub fn name(&self) -> &str {
        &self.info.script_name
    }
    
    pub fn file_name(&self) -> Option<&str> {
        Path::new(&self.info.source_file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
    }
    
    pub fn line_number(&self, pc: i32) -> i32 {
        for i in 0..self.info.pcs.len() {
            if self.info.pcs[i] == pc {
                if i > 0 {
                    return self.info.lines[i - 1];
                }
                break;
            }
        }
        
        // Default to the last line if no match found.
        self.info.lines.last().copied().unwrap_or(0)
    }
    
    pub fn decode(id: usize, mut packet: Packet) -> ScriptFile {
        let length = packet.len();
        if length < 16 {
            error!("Invalid script file (minimum length).");
        }
        
        packet.position = length - 2;
        let trailer_length = packet.g2() as usize;
        let trailer_position = length - trailer_length - 12 - 2;
        
        if trailer_position < 0 || trailer_position >= length {
            error!("Invalid script file (trailer position).");
        }
        
        packet.position = trailer_position;
        
        let mut script = ScriptFile::new(id as i32);
        let _instructions = packet.g4(); // We don't need to preallocate anything in Rust either, but still need to read it.
        
        script.int_local_count = packet.g2() as i32;
        script.string_local_count = packet.g2() as i32;
        script.int_arg_count = packet.g2() as i32;
        script.string_arg_count = packet.g2() as i32;
        
        
        let switches = packet.g1();
        for _i in 0..switches {
            let mut count = packet.g2();
            let mut table: SwitchTable = SwitchTable::new();
            
            for j in 0..count {
                let key = packet.g4();
                let offset = packet.g4();
                table.insert(key, Some(offset));
            }
            
            script.switch_tables.push(table);
        }
        
        packet.position = 0;
        script.info.script_name = packet.gjstr(0);
        script.info.source_file_path = packet.gjstr(0);
        script.info.lookup_key = packet.g4();
        debug!("script info name: {:?}", script.info.script_name);
        debug!("source path: {:?}", script.info.source_file_path);
        
        let parameter_type_count = packet.g1();
        for _i in 0..parameter_type_count {
            script.info.parameter_types.push(packet.g1() as i32);
        }
        
        let line_number_table_length = packet.g2() as usize;
        for _i in 0..line_number_table_length {
            script.info.pcs.push(packet.g4());
            script.info.lines.push(packet.g4());
        }
        
        let mut instruction = 0;
        while trailer_position > packet.position {
            let opcode = packet.g2();
            debug!("opcode: {:?}", opcode);
            debug!("remaining: {:?}", packet.remaining());
            
            if opcode == ScriptOpcode::PUSH_CONSTANT_STRING as u16 {
                let test = packet.gjstr(0);
                debug!("test: {:?}", test);
                script.string_operands.push(test);
            } else if is_large_operand(opcode as i32) {
                script.int_operands.push(packet.g4());
            } else {
                script.int_operands.push(packet.g1() as i32);
            }
            
            script.opcodes.insert(instruction, ScriptOpcode::try_from(opcode as i32).expect("Invalid opcode"));
            instruction += 1;
        }
        
        script
    }
}