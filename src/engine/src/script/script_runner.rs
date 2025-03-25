use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Instant;
use log::{debug, error};
use crate::entity::entity_queue_request::ScriptArgument;
use crate::entity::entity_type::EntityType;
use crate::script::handlers::core_ops::get_core_ops;
use crate::script::handlers::player_ops::get_player_ops;
use crate::script::script_file::ScriptFile;
use crate::script::script_pointer::ScriptPointer;
use crate::script::script_state::ScriptState;

// Function type for script handlers - requires Send+Sync for thread safety
pub type CommandHandler = fn(&mut ScriptState);

// Map of opcode numbers to handler functions
pub type CommandHandlers = HashMap<i32, CommandHandler>;

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn get_handlers() -> &'static CommandHandlers {
        static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();

        HANDLERS.get_or_init(|| {
            let mut handlers = CommandHandlers::with_capacity(1000);

            for (key, func) in get_player_ops().iter() {
                handlers.insert(*key, *func);
            }

            for (key, func) in get_core_ops().iter() {
                handlers.insert(*key, *func);
            }

            handlers
        })
    }

    #[inline]
    pub fn init(
        script: ScriptFile,
        self_entity: Option<EntityType>,
        target_entity: Option<EntityType>,
        args: Option<Vec<ScriptArgument>>
    ) -> ScriptState {
        let mut state = ScriptState::new(script, args);

        if self_entity.is_none() && target_entity.is_none() {
            return state;
        }

        if let Some(self_ent) = self_entity {
            state.self_entity = Some(self_ent);
            
            unsafe {
                match &state.self_entity {
                    Some(EntityType::Player(player)) => {
                        state.active_player = Some(player.clone());
                        state.pointers |= 1 << (ScriptPointer::ActivePlayer as usize);
                    },
                    Some(EntityType::NPC(npc)) => {
                        state.active_npc = Some(npc.clone());
                        state.pointers |= 1 << (ScriptPointer::ActiveNpc as usize);
                    },
                    Some(EntityType::Loc(loc)) => {
                        state.active_loc = Some(loc.clone());
                        state.pointers |= 1 << (ScriptPointer::ActiveLoc as usize);
                    },
                    Some(EntityType::Obj(obj)) => {
                        state.active_obj = Some(obj.clone());
                        state.pointers |= 1 << (ScriptPointer::ActiveObj as usize);
                    },
                    _ => {}
                }
            }
        }

        if let Some(target) = target_entity {
            let has_same_type = match (&state.self_entity, &target) {
                (Some(EntityType::Player(_)), EntityType::Player(_)) |
                (Some(EntityType::NPC(_)), EntityType::NPC(_)) |
                (Some(EntityType::Loc(_)), EntityType::Loc(_)) |
                (Some(EntityType::Obj(_)), EntityType::Obj(_)) => true,
                _ => false
            };
            
            unsafe {
                match target {
                    EntityType::Player(player) => {
                        if has_same_type {
                            state.active_player2 = Some(player.clone());
                            state.pointers |= 1 << (ScriptPointer::ActivePlayer2 as usize);
                        } else {
                            state.active_player = Some(player.clone());
                            state.pointers |= 1 << (ScriptPointer::ActivePlayer as usize);
                        }
                    },
                    EntityType::NPC(npc) => {
                        if has_same_type {
                            state.active_npc2 = Some(npc.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveNpc2 as usize);
                        } else {
                            state.active_npc = Some(npc.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveNpc as usize);
                        }
                    },
                    EntityType::Loc(loc) => {
                        if has_same_type {
                            state.active_loc2 = Some(loc.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveLoc2 as usize);
                        } else {
                            state.active_loc = Some(loc.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveLoc as usize);
                        }
                    },
                    EntityType::Obj(obj) => {
                        if has_same_type {
                            state.active_obj2 = Some(obj.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveObj2 as usize);
                        } else {
                            state.active_obj = Some(obj.clone());
                            state.pointers |= 1 << (ScriptPointer::ActiveObj as usize);
                        }
                    }
                }
            }
        }

        state
    }

    #[inline(always)]
    pub fn execute(
        state: &mut ScriptState,
        reset: bool,
        benchmark: bool,
    ) -> i32 {
        if reset {
            state.reset();
        }

        if state.execution != ScriptState::RUNNING {
            state.execution_history.push(state.execution);
            state.execution = ScriptState::RUNNING;
        }

        // Get profiling timer if needed
        #[cfg(feature = "profiling")]
        let start = if benchmark { Some(Instant::now()) } else { None };

        let handlers = Self::get_handlers();

        while state.execution == ScriptState::RUNNING {
            state.opcount += 1;
            state.pc += 1;

            let opcodes = &state.script.opcodes;
            let opcodes_len = opcodes.len() as i32;

            if state.pc >= opcodes_len {
                state.execution = ScriptState::FINISHED;
                return state.execution;
            }

            if !benchmark && state.opcount > 500_000 {
                state.execution = ScriptState::ABORTED;
                return state.execution;
            }

            let opcode = opcodes[state.pc as usize] as i32;

            #[cfg(debug_assertions)]
            {
                if let Some(handler) = handlers.get(&opcode) {
                    let handler_fn = *handler;
                    handler_fn(state);
                } else {
                    error!("Unknown opcode: {}", opcode);
                    state.execution = ScriptState::ABORTED;
                    return state.execution;
                }
            }

            #[cfg(not(debug_assertions))]
            {
                // Release build - use unchecked access for maximum speed
                // SAFETY: All opcodes are validated at compile-time
                let handler_fn = unsafe { *handlers.get(&opcode).unwrap_unchecked() };
                handler_fn(state);
            }
        }

        // Profiling - only if enabled and benchmarking
        #[cfg(feature = "profiling")]
        if let Some(start) = start {
            let elapsed = start.elapsed();
            let time_microseconds = elapsed.as_micros() as i32;

            if time_microseconds > 500_000 {
                let message = format!(
                    "Warning [cpu time]: Script: {}, time: {}us, opcount: {}",
                    state.script.name(), time_microseconds, state.opcount
                );

                if let Some(EntityType::Player(_)) = state.self_entity {
                    // TODO - send message to player
                } else if time_microseconds > 5000 {
                    error!("{}", message);
                }
            }

            debug!("time: {}µs, opcount: {}", time_microseconds, state.opcount);
        }

        state.execution
    }

    #[inline]
    pub fn execute_opcode(state: &mut ScriptState, opcode: i32) -> Result<(), String> {
        let handlers = Self::get_handlers();

        if let Some(handler) = handlers.get(&opcode) {
            handler(state);
            Ok(())
        } else {
            Err(format!("Unknown opcode: {}", opcode))
        }
    }
}