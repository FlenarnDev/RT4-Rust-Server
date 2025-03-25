use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::CommandHandlers;
use crate::script::script_state::ScriptState;
use std::collections::HashMap;
use std::sync::OnceLock;


#[inline(always)]
fn handle_push_constant_string(state: &mut ScriptState) {
    let string_operand = state.get_string_operand();
    state.push_string(string_operand.parse().unwrap());
}

#[inline(always)]
fn handle_return(state: &mut ScriptState) {
    if state.fp == 0 {
        state.execution = ScriptState::FINISHED;
        return;
    }
    state.pop_frame();
}

pub fn get_core_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();

    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(64); // TODO - Update as need be.

        handlers.insert(
            ScriptOpcode::PUSH_CONSTANT_STRING as i32,
            handle_push_constant_string
        );

        handlers.insert(
            ScriptOpcode::RETURN as i32,
            handle_return
        );
        handlers
    })
}