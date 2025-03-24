use crate::entity::entity::Entity;
use crate::entity::entity_queue_request::ScriptArgument;
use crate::entity::player::Player;
use crate::script::script_file::ScriptFile;
use crate::script::script_pointer::ScriptPointer;
use crate::script::server_trigger_types::ServerTriggerTypes;

#[derive(Clone)]
pub struct GosubStackFrame {
    pub script: ScriptFile,
    pub pc: i32,
    pub int_locals: Vec<i32>,
    pub string_locals: Vec<String>
}

// For debugger stack traces
#[derive(Clone)]
pub struct JumpStackFrame {
    pub script: ScriptFile,
    pub pc: i32,
}

pub struct ScriptState {
    pub script: ScriptFile,
    pub trigger: ServerTriggerTypes,
    pub execution: i32,
    pub execution_history: Vec<i32>,
    pub pc: i32,
    pub opcount: i32,
    pub frames: Vec<GosubStackFrame>,
    pub fp: usize,
    pub debug_frames: Vec<JumpStackFrame>,
    pub debug_fp: usize,
    pub int_stack: Vec<Option<i32>>,
    pub isp: usize,
    pub string_stack: Vec<Option<String>>,
    pub ssp: usize,
    pub int_locals: Vec<i32>,
    pub string_locals: Vec<String>,

    // Necessary in our implementation?
    pointers: i32,

    pub self_entity: Option<Entity>,

    // Active entities
    pub active_player: Option<Player>,
    pub active_player2: Option<Player>,
    // TODO - LOC & NPC

    // String splitting
    pub split_pages: Vec<Vec<String>>,
    pub split_mesanim: i32
}

struct ScriptTriggerTypes(i32);

impl ScriptState {
    pub const ABORTED: i32 = -1;
    pub const RUNNING: i32 = 0;
    pub const FINISHED: i32 = 1;
    pub const SUSPENDED : i32 = 2; // Suspended, move to player
    pub const PAUSEBUTTON: i32 = 3;
    pub const COUNTDIALOG: i32 = 4;
    pub const NPC_SUSPENDED: i32 = 5; // Suspended, move to NPC
    pub const LOC_SUSPENDED: i32 = 6; // Suspended, move to world

    pub fn new(script: ScriptFile, args: Option<Vec<ScriptArgument>>) -> Self {
        let mut int_locals = Vec::new();
        let mut string_locals = Vec::new();

        if let Some(arg_list) = args {
            for arg in arg_list {
                match arg {
                    ScriptArgument::Number(num) => int_locals.push(num),
                    ScriptArgument::String(string) => string_locals.push(string),
                }
            }
        }

        let script_clone = script.clone();

        ScriptState {
            script: script_clone,
            trigger: ServerTriggerTypes::try_from(script.info.lookup_key & 0xff).unwrap(),
            execution: Self::RUNNING,
            execution_history: Vec::new(),
            pc: -1,
            opcount: 0,
            frames: Vec::new(),
            fp: 0,
            debug_frames: Vec::new(),
            debug_fp: 0,
            int_stack: Vec::new(),
            isp: 0,
            string_stack: Vec::new(),
            ssp: 0,
            int_locals,
            string_locals,
            pointers: 0,
            self_entity: None,
            active_player: None,
            active_player2: None,
            split_pages: Vec::new(),
            split_mesanim: -1,
        }
    }

    pub fn pointers_set(&mut self, pointers: &[ScriptPointer]) {
        self.pointers = 0;
        for &pointer in pointers {
            self.pointers |= 1 << pointer as i32
        }
    }
    
    pub fn pointer_add(&mut self, pointer: ScriptPointer) {
        self.pointers |= 1 << pointer as i32;
    }
    
    pub fn pointer_remove(&mut self, pointer: ScriptPointer) {
        self.pointers &= !(1 << pointer as i32);
    }
    
    pub fn pointer_get(&self, pointer: ScriptPointer) -> bool {
        (self.pointers & 1 << pointer as i32) != 0
    }
    
    pub fn pointer_check(&self, pointers: &[ScriptPointer]) -> Result<(), String> {
        for &pointer in pointers {
            let flag = 1 << pointer as i32;
            if (self.pointers & flag) != flag {
                return Err(format!(
                    "Required pointer: {}, current: {}",
                    Self::pointer_print(flag),
                    Self::pointer_print(self.pointers)
                ));
            }
        }
        Ok(())
    }
    
    fn pointer_print(flags: i32) -> String {
        let mut text = String::new();
        for i in 0..ScriptPointer::_LAST as i32 {
            if (flags & 1 << i) != 0 {
                text.push_str(&format!("{:?}, ", ScriptPointer::try_from(i).unwrap()));
            }
        }
        if text.len() > 2 { 
            text.truncate(text.len() - 2); // Remove trailing ", "
        }
        text
    }
    
    pub fn get_active_player(&self) -> Result<&Player, String> {
        let player = if self.get_int_operand() == 0 {
            &self.active_player
        } else {
            &self.active_player2
        };
        
        match player {
            Some(player) => Ok(player),
            None => Err(format!("Player not found")),
        }
    }
    
    pub fn set_active_player(&mut self, player: Player) {
        if self.get_int_operand() == 0 {
            self.active_player = Some(player);
        } else {
            self.active_player2 = Some(player);
        }
    }

    pub fn get_int_operand(&self) -> i32 {
        self.script.int_operands[self.pc as usize]
    }

    pub fn get_string_operand(&self) -> &str {
        &self.script.string_operands[self.pc as usize]
    }
    
    pub fn pop_int(&mut self) -> i32 {
        self.isp -= 1;
        match self.int_stack.get(self.isp) { 
            Some(Some(value)) => *value,
            _ => 0
        }
    }
    
    pub fn pop_ints(&mut self, amount: usize) -> Vec<i32> {
        let mut ints = vec![0; amount];
        for i in (0..amount).rev() {
            ints[i] = self.pop_int();
        }
        ints
    }
    
    pub fn push_int(&mut self, value: i32) {
        if self.isp >= self.int_stack.len() {
            self.int_stack.resize(self.isp + 1, None);
        }
        self.int_stack[self.isp] = Some(value);
        self.isp += 1;
    }
    
    pub fn pop_string(&mut self) -> String {
        self.ssp -= 1;
        match self.string_stack.get(self.ssp) {
            Some(Some(value)) => value.clone(),
            _ => String::new()
        }
    }
    
    pub fn pop_strings(&mut self, amount: usize) -> Vec<String> {
        let mut strings = vec![String::new(); amount];
        for i in (0..amount).rev() {
            strings[i] = self.pop_string();
        }
        strings
    }
    
    pub fn push_string(&mut self, string: String) {
        if self.ssp >= self.string_stack.len() {
            self.string_stack.resize(self.ssp + 1, None);
        }
        self.string_stack[self.ssp] = Some(string);
        self.ssp += 1;
    }
    
    pub fn pop_frame(&mut self) {
        self.fp -= 1;
        let frame = &self.frames[self.fp];
        
        self.pc = frame.pc;
        self.script = frame.script.clone();
        
        self.int_locals = frame.int_locals.clone();
        self.string_locals = frame.string_locals.clone();
    }
    
    pub fn gosub_frame(&mut self, proc: ScriptFile) {
        if self.fp >= self.frames.len() {
            self.frames.resize(self.fp + 1, GosubStackFrame {
                script: self.script.clone(),
                pc: 0,
                int_locals: Vec::new(),
                string_locals: Vec::new(),
            });
        }
        
        self.frames[self.fp] = GosubStackFrame {
            script: self.script.clone(),
            pc: self.pc,
            int_locals: self.int_locals.clone(),
            string_locals: self.string_locals.clone(),
        };
        
        self.fp += 1;
        self.setup_new_script(proc);
    }
    
    pub fn goto_frame(&mut self, label: ScriptFile) {
        if self.debug_fp >= self.debug_frames.len() {
            self.debug_frames.resize(self.debug_fp + 1, JumpStackFrame {
                script: self.script.clone(),
                pc: 0,
            });
        }
        
        self.debug_frames[self.debug_fp] = JumpStackFrame {
            script: self.script.clone(),
            pc: self.pc,
        };
        
        self.debug_fp += 1;
        self.fp = 0;
        self.frames.clear();
        self.setup_new_script(label);
    }
    
    pub fn setup_new_script(&mut self, script: ScriptFile) {
        let mut int_locals = vec![0; script.int_local_count as usize];
        let int_arg_count = script.int_arg_count as usize;
        
        for index in 0..int_arg_count {
            int_locals[int_arg_count - index - 1] = self.pop_int();
        }
        
        let mut string_locals = vec![String::new(); script.string_local_count as usize];
        let string_arg_count = script.string_arg_count as usize;
        
        for index in 0..string_arg_count {
            string_locals[string_arg_count - index - 1] = self.pop_string();
        }
        
        self.pc = -1;
        self.script = script;
        self.int_locals = int_locals;
        self.string_locals = string_locals;
    }
    
    pub fn reset(&mut self) {
        self.pc = -1;
        self.frames.clear();
        self.fp = 0;
        self.int_stack.clear();
        self.isp = 0;
        self.string_stack.clear();
        self.ssp = 0;
        self.int_locals.clear();
        self.string_locals.clear();
        self.pointers = 0;
    }
}