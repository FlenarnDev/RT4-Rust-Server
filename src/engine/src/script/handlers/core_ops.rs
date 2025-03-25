use std::collections::HashMap;
use std::sync::OnceLock;
use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::{CommandHandler, CommandHandlers};
use crate::script::script_state::ScriptState;

pub fn get_core_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();

    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(64); // Pre-allocate capacity for better performance

        handlers.insert(
            ScriptOpcode::PUSH_CONSTANT_STRING as i32,
            Box::new(|state: &mut ScriptState| {
                let string_operand = state.get_string_operand();
                state.push_string(string_operand.to_string());
            })
        );

        handlers.insert(
            ScriptOpcode::RETURN as i32,
            Box::new(|state: &mut ScriptState| {
                if state.fp == 0 {
                    state.execution = ScriptState::FINISHED;
                    return;
                }
                state.pop_frame();
            })
        );

        // Add other opcode handlers here...

        handlers
    })
}