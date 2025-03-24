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

pub const ACTIVE_NPC: [ScriptPointer; 2] = [ScriptPointer::ActiveNpc, ScriptPointer::ActiveNpc2];
pub const ACTIVE_LOC: [ScriptPointer; 2] = [ScriptPointer::ActiveLoc, ScriptPointer::ActiveLoc2];
pub const ACTIVE_OBJ: [ScriptPointer; 2] = [ScriptPointer::ActiveObj, ScriptPointer::ActiveObj2];
pub const ACTIVE_PLAYER: [ScriptPointer; 2] = [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2];
pub const PROTECTED_ACTIVE_PLAYER: [ScriptPointer; 2] = [
    ScriptPointer::ProtectedActivePlayer,
    ScriptPointer::ProtectedActivePlayer2,
];

/// Wraps a command handler in another function that will check for pointer presence in the state.
///
/// # Arguments
///
/// * `pointer` - The pointer to check for. If it is an array, the int operand is used as the index in the array.
/// * `handler` - The handler to run after checking the pointer.
pub fn checked_handler(
    pointer: impl Into<PointerCheck>,
    handler: impl Fn(&mut ScriptState) + 'static,
) -> CommandHandler {
    let pointer_check = pointer.into();

    Box::new(move |state: &mut ScriptState| {
        match pointer_check {
            PointerCheck::Single(ptr) => state.pointer_check(&[ptr]).unwrap(),
            PointerCheck::Multiple(ptrs) => state.pointer_check(&[ptrs[state.get_int_operand() as usize]]).unwrap(),
        }

        handler(state);
    })
}

/// Helper enum to handle either a single pointer or an array of pointers
pub enum PointerCheck {
    Single(ScriptPointer),
    Multiple([ScriptPointer; 2]),
}

impl From<ScriptPointer> for PointerCheck {
    fn from(pointer: ScriptPointer) -> Self {
        PointerCheck::Single(pointer)
    }
}

impl From<[ScriptPointer; 2]> for PointerCheck {
    fn from(pointers: [ScriptPointer; 2]) -> Self {
        PointerCheck::Multiple(pointers)
    }
}