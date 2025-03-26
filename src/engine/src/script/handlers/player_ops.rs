use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::CommandHandlers;
use crate::script::script_state::ScriptState;
use std::collections::HashMap;
use std::sync::OnceLock;
use log::debug;
use crate::io::server::model::message_game::Message_Game;

#[inline(always)]
fn handle_mes(state: &mut ScriptState) {
    let message = state.pop_string();
    if let Ok(player) = state.get_active_player() {
        player.write(Message_Game::new(message));
    } else {
        debug!("No active player");
    }
}

pub fn get_player_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();
    
    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(64); // TODO - update as need be

        handlers.insert(
            ScriptOpcode::MES as i32,
            handle_mes
        );

        handlers  
    })
    
}