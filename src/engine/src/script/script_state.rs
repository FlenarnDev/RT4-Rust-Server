use std::sync::Arc;
use reqwest::header::SERVER;
use crate::entity::entity_queue_request::ScriptArgument;
use crate::entity::entity_type::EntityType;
use crate::entity::loc::Loc;
use crate::entity::npc::NPC;
use crate::entity::obj::Obj;
use crate::entity::player::Player;
use crate::script::script_file::ScriptFile;
use crate::script::script_pointer::ScriptPointer;
use crate::script::server_trigger_types::ServerTriggerTypes;

#[derive(Clone, PartialEq)]
pub struct GosubStackFrame {
    pub script: Arc<ScriptFile>,
    pub pc: i32,
    pub int_locals: Vec<i32>,
    pub string_locals: Vec<String>
}

#[derive(Clone, PartialEq)]
pub struct JumpStackFrame {
    pub script: Arc<ScriptFile>,
    pub pc: i32,
}

#[derive(Clone, PartialEq)]
pub struct ScriptState {
    pub script: Arc<ScriptFile>,
    pub trigger: ServerTriggerTypes,
    pub execution: i32,
    pub execution_history: Vec<i32>,
    pub pc: i32,
    pub opcount: i32,
    pub frames: Vec<GosubStackFrame>,
    pub fp: usize,
    pub debug_frames: Vec<JumpStackFrame>,
    pub debug_fp: usize,
    pub int_stack: Vec<i32>,
    pub isp: usize,
    pub string_stack: Vec<String>,
    pub ssp: usize,
    pub int_locals: Vec<i32>,
    pub string_locals: Vec<String>,
    pub pointers: i32,
    pub self_entity: Option<EntityType>,
    pub active_player: Option<Player>,
    pub active_player2: Option<Player>,
    pub active_npc: Option<NPC>,
    pub active_npc2: Option<NPC>,
    pub active_loc: Option<Loc>,
    pub active_loc2: Option<Loc>,
    pub active_obj: Option<Obj>,
    pub active_obj2: Option<Obj>,
    pub split_pages: Vec<Vec<String>>,
    pub split_mesanim: i32
}

impl ScriptState {
    pub const ABORTED: i32 = -1;
    pub const RUNNING: i32 = 0;
    pub const FINISHED: i32 = 1;
    pub const SUSPENDED: i32 = 2;
    pub const PAUSEBUTTON: i32 = 3;
    pub const COUNTDIALOG: i32 = 4;
    pub const NPC_SUSPENDED: i32 = 5;
    pub const WORLD_SUSPENDED: i32 = 6;

    pub fn new(script: ScriptFile, args: Option<Vec<ScriptArgument>>) -> Self {
        let mut int_locals = Vec::new();
        let mut string_locals = Vec::new();

        if let Some(arg_list) = args {
            int_locals.reserve(arg_list.len());
            string_locals.reserve(arg_list.len());
            
            for arg in arg_list {
                match arg {
                    ScriptArgument::Number(num) => int_locals.push(num),
                    ScriptArgument::String(string) => string_locals.push(string),
                }
            }
        }

        let arc_script = Arc::new(script);
        let trigger = ServerTriggerTypes::try_from(arc_script.info.lookup_key & 0xFF).unwrap();

        ScriptState {
            script: arc_script,
            trigger,
            execution: Self::RUNNING,
            execution_history: Vec::new(),
            pc: -1,
            opcount: 0,
            frames: Vec::with_capacity(8),
            fp: 0,
            debug_frames: Vec::with_capacity(4),
            debug_fp: 0,
            int_stack: Vec::with_capacity(32),
            isp: 0,
            string_stack: Vec::with_capacity(32),
            ssp: 0,
            int_locals,
            string_locals,
            pointers: 0,
            self_entity: None,
            active_player: None,
            active_player2: None,
            active_npc: None,
            active_npc2: None,
            active_loc: None,
            active_loc2: None,
            active_obj: None,
            active_obj2: None,
            split_pages: Vec::new(),
            split_mesanim: -1,
        }
    }

    pub fn pointers_set(&mut self, pointers: &[ScriptPointer]) {
        self.pointers = pointers.iter().fold(0, |acc, &p| acc | 1 << p as i32);
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
        (0..ScriptPointer::_LAST as i32)
            .filter(|&i| flags & (1 << i) != 0)
            .map(|i| format!("{:?}", ScriptPointer::try_from(i).unwrap()))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn get_active_player(&self) -> Result<&Player, String> {
        match if self.get_int_operand() == 0 {
            &self.active_player
        } else {
            &self.active_player2
        } {
            Some(player) => Ok(player),
            None => Err("Player not found".to_string()),
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
        self.int_stack.get(self.isp).copied().unwrap_or(0)
    }

    pub fn pop_ints(&mut self, amount: usize) -> Vec<i32> {
        (0..amount).rev().map(|_| self.pop_int()).collect()
    }

    pub fn push_int(&mut self, value: i32) {
        if self.isp < self.int_stack.len() {
            self.int_stack[self.isp] = value;
        } else {
            self.int_stack.push(value);
        }
        self.isp += 1;
    }

    pub fn pop_string(&mut self) -> String {
        self.ssp -= 1;
        self.string_stack.get(self.ssp).cloned().unwrap_or_default()
    }

    pub fn pop_strings(&mut self, amount: usize) -> Vec<String> {
        (0..amount).rev().map(|_| self.pop_string()).collect()
    }

    pub fn push_string(&mut self, string: String) {
        if self.ssp < self.string_stack.len() {
            self.string_stack[self.ssp] = string;
        } else {
            self.string_stack.push(string);
        }
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
            self.frames.push(GosubStackFrame {
                script: Arc::clone(&self.script),
                pc: 0,
                int_locals: Vec::new(),
                string_locals: Vec::new(),
            });
        }
        
        let mut new_int_locals = Vec::with_capacity(self.int_locals.len());
        new_int_locals.extend_from_slice(&mut self.int_locals);

        let mut new_string_locals = Vec::with_capacity(self.string_locals.len());
        for s in &self.string_locals {
            new_string_locals.push(s.clone());
        }

        self.frames[self.fp] = GosubStackFrame {
            script: Arc::clone(&self.script),
            pc: self.pc,
            int_locals: new_int_locals,
            string_locals: new_string_locals,
        };

        self.fp += 1;
        self.setup_new_script(proc);
    }

    pub fn goto_frame(&mut self, label: ScriptFile) {
        if self.debug_fp >= self.debug_frames.len() {
            self.debug_frames.push(JumpStackFrame {
                script: Arc::clone(&self.script),
                pc: 0,
            });
        }

        self.debug_frames[self.debug_fp] = JumpStackFrame {
            script: Arc::clone(&self.script),
            pc: self.pc,
        };

        self.debug_fp += 1;
        self.fp = 0;
        self.frames.clear();
        self.setup_new_script(label);
    }

    pub fn setup_new_script(&mut self, script: ScriptFile) {
        let arc_script = Arc::new(script);

        let int_local_count = arc_script.int_local_count as usize;
        let string_local_count = arc_script.string_local_count as usize;
        let int_arg_count = arc_script.int_arg_count as usize;
        let string_arg_count = arc_script.string_arg_count as usize;
        
        let mut int_locals = vec![0; int_local_count];
        let mut string_locals = vec![String::new(); string_local_count];

        for i in (0..int_arg_count).rev() {
            int_locals[i] = self.pop_int();
        }

        for i in (0..string_arg_count).rev() {
            string_locals[i] = self.pop_string();
        }

        self.pc = -1;
        self.script = arc_script;
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