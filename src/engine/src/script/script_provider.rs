use std::collections::HashMap;
use crate::script::script_file::ScriptFile;

// Maintains a list of scripts (id <-> name)
pub struct ScriptProvider {
    scripts: Vec<ScriptFile>,
    script_lookup: HashMap<u32, ScriptFile>,
    script_names: HashMap<String, u32>,
}

impl ScriptProvider {
    pub const COMPILER_VERSION: u32 = 23;
}