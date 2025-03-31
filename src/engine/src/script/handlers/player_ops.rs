use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::CommandHandlers;
use crate::script::script_state::ScriptState;
use std::collections::HashMap;
use std::sync::OnceLock;
use crate::engine::Engine;
use crate::io::server::model::message_game::Message_Game;

pub fn get_player_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();
    
    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(64); // TODO - update as need be

        handlers.insert(
            ScriptOpcode::MES as i32,
            |state: &mut ScriptState| {
                let pid = state.get_active_player().expect("No active player found").get_pid();
                let player = Engine::get().players.get_mut(pid).expect(format!("No player found for PID: {}", pid).as_str());
                player.write(Message_Game::new(state.pop_string()));
            }
        );

        handlers  
    })
}