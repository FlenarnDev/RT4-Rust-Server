use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::CommandHandlers;
use crate::script::script_state::ScriptState;
use std::collections::HashMap;
use std::sync::OnceLock;
use log::error;
use rsmod::changeLoc;
use crate::script::script_file::{ScriptFile, SwitchTable};
use crate::script::script_provider::ScriptProvider;

pub fn get_core_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();

    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(64); // TODO - Update as need be.

        handlers.insert(
            ScriptOpcode::PUSH_CONSTANT_INT as i32,
            |state: &mut ScriptState| {
                state.push_int(state.get_int_operand())
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_VARP as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_VARP as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );

        handlers.insert(
            ScriptOpcode::PUSH_CONSTANT_STRING as i32,
            |state: &mut ScriptState| {
                let string_operand = state.get_string_operand();
                state.push_string(string_operand.parse().unwrap());
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_VARN as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_VARN as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        handlers.insert(
            ScriptOpcode::BRANCH as i32,
            |state: &mut ScriptState| {
                state.pc += state.get_int_operand();
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_NOT as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a != b {
                    state.pc += state.get_int_operand();
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_EQUALS as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a == b {
                    state.pc += state.get_int_operand();
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_LESS_THAN as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a < b {
                    state.pc += state.get_int_operand();
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_GREATER_THAN as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a > b {
                    state.pc += state.get_int_operand();
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_VARS as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_VARS as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );

        handlers.insert(
            ScriptOpcode::RETURN as i32,
            |state: &mut ScriptState| {
                if state.fp == 0 {
                    state.execution = ScriptState::FINISHED;
                    return;
                }
                state.pop_frame();
            },
        );

        handlers.insert(
            ScriptOpcode::GOSUB as i32,
            |state: &mut ScriptState| {
                if state.fp >= 50 {
                    error!("Stack overflow");
                }

                let proc: Option<ScriptFile> = ScriptProvider::get(state.pop_int() as usize);
                if let Some(proc) = proc {
                    state.gosub_frame(proc)
                } else {
                    error!("Unable to find proc: {:?}", proc);
                }
            }
        );

        handlers.insert(
            ScriptOpcode::JUMP as i32,
            |state: &mut ScriptState| {
                let label: Option<ScriptFile> = ScriptProvider::get(state.pop_int() as usize);
                if label.is_some() {
                    error!("Unable to find label: {:?}", label);
                }
                state.goto_frame(label.unwrap());
            }
        );
        
        handlers.insert(
            ScriptOpcode::SWITCH as i32,
            |state: &mut ScriptState| {
                /*let key = state.pop_int();
                let operand = state.get_int_operand() as usize;
                let table: Option<SwitchTable> = &state.script.switch_tables[operand];
                
                if table.is_none() {
                    return;
                }
                
                let result = table.unwrap().get(&key);
                if let Some(result) = result {
                    state.pc += result.unwrap();
                }*/
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_VARBIT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_VARBIT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_LESS_THAN_OR_EQUALS as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a <= b {
                    state.pc += state.get_int_operand();
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::BRANCH_GREATER_THAN_OR_EQUALS as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();
                
                if a >= b {
                    state.pc += state.get_int_operand();
                }
            }
        );

        handlers.insert(
            ScriptOpcode::PUSH_INT_LOCAL as i32,
            |state: &mut ScriptState| {
                state.push_int(state.int_locals[state.get_int_operand() as usize])
            }
        );

        handlers.insert(
            ScriptOpcode::POP_INT_LOCAL as i32,
            |state: &mut ScriptState| {
                let operand = state.get_int_operand() as usize; 
                state.int_locals[operand] = state.pop_int();
            }
        );

        /*handlers.insert(
            ScriptOpcode::PUSH_STRING_LOCAL as i32,
            |state: &mut ScriptState| {
                state.push_string(std::mem::replace(&mut state.string_locals[state.get_int_operand() as usize], String::new()));
            }
        );*/
        
        /*handlers.insert(
            ScriptOpcode::POP_STRING_LOCAL as i32,
            |state: &mut ScriptState| {
                state.string_locals[state.get_int_operand() as usize] = state.pop_string();
            }
        );*/
        
        handlers.insert(
            ScriptOpcode::JOIN_STRING as i32,
            |state: &mut ScriptState| {
                let count = state.get_int_operand();
                
                let mut strings = Vec::with_capacity(count as usize);
                for _i in 0..count {
                    strings.push(state.pop_string());
                }
                strings.reverse();
                state.push_string(strings.join(""));
            }
        );

        handlers.insert(
            ScriptOpcode::POP_INT_DISCARD as i32,
            |state: &mut ScriptState| {
                state.isp -= 1
            }
        );

        handlers.insert(
            ScriptOpcode::POP_STRING_DISCARD as i32,
            |state: &mut ScriptState| {
                state.ssp -= 1
            }
        );
        
        handlers.insert(
            ScriptOpcode::GOSUB_WITH_PARAMS as i32,
            |state: &mut ScriptState| {
                if state.fp >= 50 {
                    error!("Stack overflow");
                }

                let proc: Option<ScriptFile> = ScriptProvider::get(state.get_int_operand() as usize);
                if let Some(proc) = proc {
                    state.gosub_frame(proc)
                } else {
                    error!("Unable to find proc: {:?}", proc);
                }
            }
        );
        
        handlers.insert(
            ScriptOpcode::JUMP_WITH_PARAMS as i32,
            |state: &mut ScriptState| {
                let label: Option<ScriptFile> = ScriptProvider::get(state.get_int_operand() as usize);
                if label.is_some() {
                    error!("Unable to find label: {:?}", label);
                }
                state.goto_frame(label.unwrap());
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_VARC_INT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_VARC_INT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::DEFINE_ARRAY as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::PUSH_ARRAY_INT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers.insert(
            ScriptOpcode::POP_ARRAY_INT as i32,
            |state: &mut ScriptState| {
                error!("Unimplemented");
            }
        );
        
        handlers
    })
}