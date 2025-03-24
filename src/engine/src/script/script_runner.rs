use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;
use log::error;
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
        
        state.self_entity = self_entity.clone();
        
        if let Some(entity) = &self_entity {
            match entity {
                EntityType::Player(player) => {
                    state.active_player = Some(player.clone());
                    state.pointer_add(ScriptPointer::ActivePlayer);
                },
                EntityType::NPC(npc) => {
                    state.active_npc = Some(npc.clone());
                    state.pointer_add(ScriptPointer::ActiveNpc);
                },
                EntityType::Loc(loc) => {
                    state.active_loc = Some(loc.clone());
                    state.pointer_add(ScriptPointer::ActiveLoc);
                },
                EntityType::Obj(obj) => {
                    state.active_obj = Some(obj.clone());
                    state.pointer_add(ScriptPointer::ActiveObj);
                }
            }
        }
        
        if let Some(target) = &target_entity {
            match (self_entity.as_ref(), target) {
                (_, EntityType::Player(player)) => {
                    match self_entity.as_ref() {
                        Some(EntityType::Player(_)) => {
                            state.active_player2 = Some(player.clone());
                            state.pointer_add(ScriptPointer::ActivePlayer2);
                        },
                        _ => {
                            state.active_player = Some(player.clone());
                            state.pointer_add(ScriptPointer::ActivePlayer);
                        }
                    }
                },

                // Npc target
                (_, EntityType::NPC(npc)) => {
                    match self_entity.as_ref() {
                        Some(EntityType::NPC(_)) => {
                            state.active_npc2 = Some(npc.clone());
                            state.pointer_add(ScriptPointer::ActiveNpc2);
                        },
                        _ => {
                            state.active_npc = Some(npc.clone());
                            state.pointer_add(ScriptPointer::ActiveNpc);
                        }
                    }
                },

                // Loc target
                (_, EntityType::Loc(loc)) => {
                    match self_entity.as_ref() {
                        Some(EntityType::Loc(_)) => {
                            state.active_loc2 = Some(loc.clone());
                            state.pointer_add(ScriptPointer::ActiveLoc2);
                        },
                        _ => {
                            state.active_loc = Some(loc.clone());
                            state.pointer_add(ScriptPointer::ActiveLoc);
                        }
                    }
                },

                // Obj target
                (_, EntityType::Obj(obj)) => {
                    match self_entity.as_ref() {
                        Some(EntityType::Obj(_)) => {
                            state.active_obj2 = Some(obj.clone());
                            state.pointer_add(ScriptPointer::ActiveObj2);
                        },
                        _ => {
                            state.active_obj = Some(obj.clone());
                            state.pointer_add(ScriptPointer::ActiveObj);
                        }
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
        let result = (|| -> Result<(), String> {
            if reset {
                state.reset();
            }
            
            if state.execution != ScriptState::RUNNING {
                state.execution_history.push(state.execution);
            }
            state.execution = ScriptState::RUNNING;
            
            let start = Instant::now();
            
            while state.execution == ScriptState::RUNNING {
                if state.pc >= state.script.opcodes.len() as i32 || state.pc < -1 {
                    return Err(format!("Invalid program counter: {}, max expected: {}",
                                       state.pc, state.script.opcodes.len()));
                }
                
                // Check opcount limit
                
                if !benchmark && state.opcount > 500_000 { 
                    return Err("Too many instructions".to_string());
                }
                
                state.opcount += 1;
                state.pc += 1;
                
                let opcode = state.script.opcodes[state.pc as usize];
                Self::execute_inner(state, i32::from(opcode))?;
            }
            
            // Handle timing/profiling as needed
            let elapsed = start.elapsed();
            let time_microseconds = elapsed.as_micros() as i32;
            
            if time_microseconds > 1000 {
                let message = format!(
                    "Warning [cpu time]: Script: {}, time: {}us, opcount: {}",
                    state.script.name(), time_microseconds, state.opcount
                );
                
                if let Some(ref entity) = state.self_entity {
                    match entity { 
                        EntityType::Player(player) => {
                            // TODO - send message to player
                            // Upcast needed here somehow...
                        },
                        _ => {
                            error!("{}", message);
                        },
                    }
                }
            }
            
            Ok(())
        })();
        
        if let Err(err) = result {
            if state.pc >= 0 && state.pc < state.script.opcodes.len() as i32 {
                error!("{}", err);
            }
            state.execution = ScriptState::ABORTED;
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