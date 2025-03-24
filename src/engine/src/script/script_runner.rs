use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;
use log::{debug, error};
use crate::entity::entity_queue_request::ScriptArgument;
use crate::entity::entity_type::EntityType;
use crate::script::handlers::core_ops::get_core_ops;
use crate::script::handlers::player_ops::get_player_ops;
use crate::script::script_file::ScriptFile;
use crate::script::script_pointer::ScriptPointer;
use crate::script::script_state::ScriptState;

/// Function type for handling script commands
pub type CommandHandler = Box<dyn Fn(&mut ScriptState)>;

/// Map of opcode numbers to their handler functions.
pub type CommandHandlers = HashMap<i32, CommandHandler>;

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn get_handlers() -> &'static CommandHandlers {
        // Define a thread-local for storing handlers
        thread_local! {
            static HANDLERS: RefCell<Option<Box<CommandHandlers>>> = RefCell::new(None);
        }

        // This is a static reference that will be created during the first call
        static mut STATIC_REF: Option<&'static CommandHandlers> = None;

        unsafe {
            if STATIC_REF.is_none() {
                // We haven't initialized yet, do it now
                let thread_handlers = Box::new(Self::create_handlers());

                // We need a reference that outlives the function
                // This is unsafe but necessary for this pattern
                let leaked_handlers: &'static CommandHandlers = Box::leak(thread_handlers);

                // Store the static reference
                STATIC_REF = Some(leaked_handlers);
            }

            STATIC_REF.unwrap()
        }
    }

    // Helper to create and populate a new handlers map
    fn create_handlers() -> CommandHandlers {
        let mut handlers = CommandHandlers::new();

        // Create a new boxed function for each handler from player_ops
        for (key, value) in get_player_ops().iter() {
            let cloned_handler: CommandHandler = Box::new(move |state| {
                (*value)(state);
            });

            handlers.insert(*key, cloned_handler);
        }

        for (key, value) in get_core_ops().iter() {
            let cloned_handler: CommandHandler = Box::new(move |state| {
                (*value)(state);
            });

            handlers.insert(*key, cloned_handler);
        }

        handlers
    }

    pub fn init(
        script: ScriptFile,
        self_entity: Option<EntityType>,
        target_entity: Option<EntityType>,
        args: Option<Vec<ScriptArgument>>
    ) -> ScriptState {
        let mut state = ScriptState::new(script, args);

        // Handle the self entity if provided
        if let Some(entity) = self_entity {
            // Store entity in state
            state.self_entity = Some(entity.clone());

            // Set active entities based on type
            match entity {
                EntityType::Player(player) => {
                    state.active_player = Some(player);
                    state.pointer_add(ScriptPointer::ActivePlayer);
                },
                EntityType::NPC(npc) => {
                    state.active_npc = Some(npc);
                    state.pointer_add(ScriptPointer::ActiveNpc);
                },
                EntityType::Loc(loc) => {
                    state.active_loc = Some(loc);
                    state.pointer_add(ScriptPointer::ActiveLoc);
                },
                EntityType::Obj(obj) => {
                    state.active_obj = Some(obj);
                    state.pointer_add(ScriptPointer::ActiveObj);
                }
            }
        }

        // Process target entity if provided
        if let Some(target) = target_entity {
            match (&state.self_entity, target) {
                // Player target
                (_, EntityType::Player(player)) => {
                    if matches!(state.self_entity, Some(EntityType::Player(_))) {
                        state.active_player2 = Some(player);
                        state.pointer_add(ScriptPointer::ActivePlayer2);
                    } else {
                        state.active_player = Some(player);
                        state.pointer_add(ScriptPointer::ActivePlayer);
                    }
                },

                // NPC target
                (_, EntityType::NPC(npc)) => {
                    if matches!(state.self_entity, Some(EntityType::NPC(_))) {
                        state.active_npc2 = Some(npc);
                        state.pointer_add(ScriptPointer::ActiveNpc2);
                    } else {
                        state.active_npc = Some(npc);
                        state.pointer_add(ScriptPointer::ActiveNpc);
                    }
                },

                // Location target
                (_, EntityType::Loc(loc)) => {
                    if matches!(state.self_entity, Some(EntityType::Loc(_))) {
                        state.active_loc2 = Some(loc);
                        state.pointer_add(ScriptPointer::ActiveLoc2);
                    } else {
                        state.active_loc = Some(loc);
                        state.pointer_add(ScriptPointer::ActiveLoc);
                    }
                },

                // Object target
                (_, EntityType::Obj(obj)) => {
                    if matches!(state.self_entity, Some(EntityType::Obj(_))) {
                        state.active_obj2 = Some(obj);
                        state.pointer_add(ScriptPointer::ActiveObj2);
                    } else {
                        state.active_obj = Some(obj);
                        state.pointer_add(ScriptPointer::ActiveObj);
                    }
                },
            }
        }

        state
    }

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

        // Profiling setup - only measure if needed
        #[cfg(feature = "profiling")]
        let start = Instant::now();

        // Check initial PC bounds
        if state.pc >= state.script.opcodes.len() as i32 || state.pc < -1 {
            error!("Invalid program counter: {}, max expected: {}", 
               state.pc, state.script.opcodes.len());
            state.execution = ScriptState::ABORTED;
            return state.execution;
        }

        // Get handlers reference once, outside the loop
        let handlers = Self::get_handlers();
        let opcodes_len = state.script.opcodes.len() as i32;

        // Main execution loop
        while state.execution == ScriptState::RUNNING {
            // Check opcount limit before incrementing PC
            if !benchmark && state.opcount > 500_000 {
                error!("Too many instructions");
                state.execution = ScriptState::ABORTED;
                return state.execution;
            }

            // Update counters
            state.opcount += 1;
            state.pc += 1;

            // Check PC bounds (only needed now since we're incrementing by 1)
            if state.pc >= opcodes_len {
                error!("Program counter out of bounds: {}", state.pc);
                state.execution = ScriptState::ABORTED;
                return state.execution;
            }

            // Fetch and execute opcode
            let opcode = state.script.opcodes[state.pc as usize] as i32; // Convert once

            // Execute opcode
            if let Some(handler) = handlers.get(&opcode) {
                if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handler(state);
                })) {
                    error!("Handler panicked for opcode {}", opcode);
                    state.execution = ScriptState::ABORTED;
                    return state.execution;
                }
            } else {
                error!("Unknown opcode: {}", opcode);
                state.execution = ScriptState::ABORTED;
                return state.execution;
            }
        }

        // Profiling - only if enabled
        #[cfg(feature = "profiling")]
        {
            let elapsed = start.elapsed();
            let time_microseconds = elapsed.as_micros() as i32;

            if time_microseconds > 1000 {
                let message = format!(
                    "Warning [cpu time]: Script: {}, time: {}us, opcount: {}",
                    state.script.name(), time_microseconds, state.opcount
                );

                if let Some(ref entity) = state.self_entity {
                    if let EntityType::Player(_) = entity {
                        // TODO - send message to player
                    } else {
                        error!("{}", message);
                    }
                }
            }
            debug!("time: {}µs, opcount: {}", time_microseconds, state.opcount);
        }

        state.execution
    }

    fn execute_inner(state: &mut ScriptState, opcode: i32) -> Result<(), String> {
        let handlers = Self::get_handlers();

        match handlers.get(&opcode) {
            Some(handler) => {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handler(state);
                })).map_err(|_| format!("Handler panicked for opcode {}", opcode))?;
                Ok(())
            },
            None => {
                Err(format!("Unknown opcode: {}", opcode))
            }
        }
    }
}