use std::collections::HashMap;
use lazy_static::lazy_static;
use log::debug;
use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_pointer::{checked_handler, ACTIVE_PLAYER};
use crate::script::script_runner::CommandHandlers;

pub fn get_player_ops() -> &'static CommandHandlers {
    static mut HANDLERS: Option<CommandHandlers> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    unsafe {
        INIT.call_once(|| {
            let mut handlers = HashMap::new();

            handlers.insert(ScriptOpcode::MES as u32, checked_handler(
                ACTIVE_PLAYER,
                |state| {
                    let message = state.pop_string();
                    let player = state.get_active_player().expect("Active player should be available");
                    debug!("Handling active player message: {:?}", message);
                    //player.message_game(&message);
                }
            ));


            HANDLERS = Some(handlers);
        });

        HANDLERS.as_ref().unwrap()
    }
}