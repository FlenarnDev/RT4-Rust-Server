use std::collections::HashMap;
use std::sync::Once;
use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::{CommandHandler, CommandHandlers};
use crate::script::script_state::ScriptState;

pub fn get_core_ops() -> &'static CommandHandlers {
    static mut HANDLERS: Option<CommandHandlers> = None;
    
    static INIT: Once = Once::new();
    
    unsafe {
        INIT.call_once(|| {
            let mut handlers = HashMap::new();

            handlers.insert(
                ScriptOpcode::PUSH_CONSTANT_STRING as i32,
                Box::new(|state: &mut ScriptState| {
                    let string_operand = state.get_string_operand().to_string();
                    state.push_string(string_operand);
                }) as Box<dyn Fn(&mut ScriptState)>
            );
            
            handlers.insert(
                ScriptOpcode::RETURN as i32,
                Box::new(|state: &mut ScriptState| {
                    if state.fp == 0 {
                        state.execution = ScriptState::FINISHED;
                        return
                    }
                    state.pop_frame();
                })
            );
            
            HANDLERS = Some(handlers);
        });
        
        HANDLERS.as_ref().unwrap()
    }
}