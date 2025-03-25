use crate::script::script_runner::CommandHandler;
use crate::script::script_state::ScriptState;
use num_enum::TryFromPrimitive;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum ScriptPointer {
    ActivePlayer,
    ActivePlayer2,
    ProtectedActivePlayer,
    ProtectedActivePlayer2,
    ActiveNpc,
    ActiveNpc2,
    ActiveLoc,
    ActiveLoc2,
    ActiveObj,
    ActiveObj2,
    _LAST,
}

// Constant arrays defined outside for zero-cost abstraction
pub const ACTIVE_NPC: [ScriptPointer; 2] = [ScriptPointer::ActiveNpc, ScriptPointer::ActiveNpc2];
pub const ACTIVE_LOC: [ScriptPointer; 2] = [ScriptPointer::ActiveLoc, ScriptPointer::ActiveLoc2];
pub const ACTIVE_OBJ: [ScriptPointer; 2] = [ScriptPointer::ActiveObj, ScriptPointer::ActiveObj2];
pub const ACTIVE_PLAYER: [ScriptPointer; 2] = [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2];
pub const PROTECTED_ACTIVE_PLAYER: [ScriptPointer; 2] = [
    ScriptPointer::ProtectedActivePlayer,
    ScriptPointer::ProtectedActivePlayer2,
];

/// Helper enum to handle either a single pointer or an array of pointers
pub enum PointerCheck {
    Single(ScriptPointer),
    Multiple([ScriptPointer; 2]),
}

impl From<ScriptPointer> for PointerCheck {
    #[inline]
    fn from(pointer: ScriptPointer) -> Self {
        PointerCheck::Single(pointer)
    }
}

impl From<[ScriptPointer; 2]> for PointerCheck {
    #[inline]
    fn from(pointers: [ScriptPointer; 2]) -> Self {
        PointerCheck::Multiple(pointers)
    }
}

/// Wraps a command handler in another function that will check for pointer presence in the state.
///
/// # Arguments
///
/// * `pointer` - The pointer to check for. If it is an array, the int operand is used as the index in the array.
/// * `handler` - The handler to run after checking the pointer.
#[inline]
pub fn checked_handler(
    pointer: impl Into<PointerCheck>,
    handler: impl Fn(&mut ScriptState) + 'static + Send + Sync,
) -> CommandHandler {
    let pointer_check = pointer.into();

    Box::new(move |state: &mut ScriptState| {
        // Perform the pointer check based on type
        match pointer_check {
            PointerCheck::Single(ptr) => {
                if let Err(e) = state.pointer_check(&[ptr]) {
                    // Handle error - log or abort
                    state.execution = ScriptState::ABORTED;
                    return;
                }
            },
            PointerCheck::Multiple(ptrs) => {
                let idx = state.get_int_operand() as usize;
                if idx >= ptrs.len() {
                    // Handle index out of bounds error
                    state.execution = ScriptState::ABORTED;
                    return;
                }

                if let Err(e) = state.pointer_check(&[ptrs[idx]]) {
                    // Handle error - log or abort
                    state.execution = ScriptState::ABORTED;
                    return;
                }
            },
        }

        // Call the handler if pointer check passes
        handler(state);
    })
}