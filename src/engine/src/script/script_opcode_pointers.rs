use std::collections::HashMap;
use crate::script::script_opcode::ScriptOpcode;

const POINTER_GROUP_FIND: [&str; 5] = ["find_player", "find_npc", "find_loc", "fond_obj", "find_db"];

#[derive(Debug, Default)]
struct ScriptOpcodePointers {
    require: Option<Vec<String>>,
    set: Option<Vec<String>>,
    corrupt: Option<Vec<String>>,
    require2: Option<Vec<String>>,
    set2: Option<Vec<String>>,
    corrupt2: Option<Vec<String>>,
    conditional: Option<bool>,
}

impl ScriptOpcodePointers {
    fn new() -> Self {
        ScriptOpcodePointers::default()
    }

    fn require(mut self, require: &[&str]) -> Self {
        self.require = Some(require.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn require2(mut self, require2: &[&str]) -> Self {
        self.require2 = Some(require2.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn set(mut self, set: &[&str]) -> Self {
        self.set = Some(set.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn set2(mut self, set2: &[&str]) -> Self {
        self.set2 = Some(set2.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn corrupt(mut self, corrupt: &[&str]) -> Self {
        self.corrupt = Some(corrupt.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn corrupt2(mut self, corrupt2: &[&str]) -> Self {
        self.corrupt2 = Some(corrupt2.iter().map(|&s| s.to_string()).collect());
        self
    }

    fn conditional(mut self, conditional: bool) -> Self {
        self.conditional = Some(conditional);
        self
    }
}

macro_rules! script_opcode {
    ($name:expr, { $( $field:ident : [ $( $value:expr ),* ] ),* $(,)? }) => {
        (
            $name,
            ScriptOpcodePointers::new()
                $(.$field(&[$($value),*]))*
        )
    };
}
fn initialize_script_opcode_pointers() {
    let mut script_opcode_pointers: HashMap<ScriptOpcode, ScriptOpcodePointers> = HashMap::new();

    let opcodes = vec![
        script_opcode!(ScriptOpcode::ALLOWDESIGN, { require: ["active_player"] }),
        script_opcode!(ScriptOpcode::ANIM, { require: ["active_player"], require2: ["active_player2"] }),
    ];
    for (name, opcode) in opcodes {
        script_opcode_pointers.insert(name, opcode);
    }
}