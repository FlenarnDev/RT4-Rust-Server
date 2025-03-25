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

// Define array constants outside functions for zero-cost
pub const ACTIVE_NPC: [ScriptPointer; 2] = [ScriptPointer::ActiveNpc, ScriptPointer::ActiveNpc2];
pub const ACTIVE_LOC: [ScriptPointer; 2] = [ScriptPointer::ActiveLoc, ScriptPointer::ActiveLoc2];
pub const ACTIVE_OBJ: [ScriptPointer; 2] = [ScriptPointer::ActiveObj, ScriptPointer::ActiveObj2];
pub const ACTIVE_PLAYER: [ScriptPointer; 2] = [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2];
pub const PROTECTED_ACTIVE_PLAYER: [ScriptPointer; 2] = [
    ScriptPointer::ProtectedActivePlayer,
    ScriptPointer::ProtectedActivePlayer2,
];