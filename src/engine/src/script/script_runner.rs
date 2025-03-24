use std::collections::HashMap;
use crate::script::script_state::ScriptState;

/// Function type for handling script commands
pub type CommandHandler = Box<dyn Fn(&mut ScriptState)>;

/// Map of opcode numbers to their handler functions.
pub type CommandHandlers = HashMap<u32, CommandHandler>;