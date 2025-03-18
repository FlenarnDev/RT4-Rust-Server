use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use log::Level::Debug;
use crate::script::script_opcode::ScriptOpcode;

pub struct ScriptInfo {
    script_name: String,
    source_file_path: String,
    lookup_key: i32,
    parameter_types: Vec<i32>,
    pcs: Vec<i32>,
    lines: Vec<i32>,
}

pub type SwitchTable = HashMap<i32, Option<i32>>;

pub fn is_large_operand(opcode: ScriptOpcode) -> bool {
    if opcode as i32 > 100 {
        return false;
    }
    
    match opcode { 
        ScriptOpcode::RETURN |
        ScriptOpcode::POP_INT_DISCARD |
        ScriptOpcode::POP_STRING_DISCARD |
        ScriptOpcode::GOSUB |
        ScriptOpcode::JUMP => false,
        _ => true,
    }
}

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
}