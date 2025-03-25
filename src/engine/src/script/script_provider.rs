use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use log::{debug, error};
use crate::io::packet::Packet;
use crate::script::script_file::ScriptFile;
use crate::script::server_trigger_types::ServerTriggerTypes;

// Static storage using OnceLock for thread safety
static SCRIPTS: OnceLock<Vec<ScriptFile>> = OnceLock::new();
static SCRIPT_LOOKUP: OnceLock<HashMap<usize, ScriptFile>> = OnceLock::new();
static SCRIPT_NAMES: OnceLock<HashMap<String, usize>> = OnceLock::new();

pub struct ScriptProvider;

impl ScriptProvider {
    pub const COMPILER_VERSION: u32 = 23;

    #[inline]
    fn ensure_initialized() {
        SCRIPTS.get_or_init(|| Vec::new());
        SCRIPT_LOOKUP.get_or_init(|| HashMap::new());
        SCRIPT_NAMES.get_or_init(|| HashMap::new());
    }

    pub fn load() -> u32 {
        let dat_path = "./data/pack/server/script.dat";
        let idx_path = "./data/pack/server/script.idx";

        match (Packet::io(dat_path.parse().unwrap()), Packet::io(idx_path.parse().unwrap())) {
            (Ok(dat), Ok(idx)) => {
                let count = Self::parse(dat, idx);
                debug!("Loaded {} scripts", count);
                count
            },
            _ => {
                error!("Failed to load script data or index files");
                0
            }
        }
    }

    fn parse(mut dat: Packet, mut idx: Packet) -> u32 {
        if dat.is_empty() || idx.is_empty() {
            error!("No scripts data found, rebuild scripts.");
            return 0;
        }

        let entries = dat.g4();
        idx.position += 4;

        let version = dat.g4();
        if version as u32 != Self::COMPILER_VERSION {
            error!("Scripts were compiled with an incompatible RuneScript compiler.");
        }

        // Pre-allocate with capacity for better performance
        let mut scripts: Vec<ScriptFile> = Vec::with_capacity(entries as usize);
        let mut script_names: HashMap<String, usize> = HashMap::with_capacity(entries as usize);
        let mut script_lookup: HashMap<usize, ScriptFile> = HashMap::with_capacity(entries as usize);

        let mut loaded = 0;
        for id in 0..entries {
            let size = idx.g4();
            if size == 0 {
                continue;
            }

            match (|| {
                let bytes = dat.gbytes(size as usize);
                let script = ScriptFile::decode(id as usize, Packet::from(bytes));

                if script.info.lookup_key != -1 {
                    script_lookup.insert(script.info.lookup_key as usize, script.clone());
                }

                script_names.insert(script.info.script_name.clone(), id as usize);
                scripts.push(script);

                loaded += 1;
                Ok::<_, Box<dyn Error>>(())
            })() {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error: {}", err);
                    eprintln!("Warning: Failed to load script {}, something may have been partially written", id);
                    return 0;
                }
            }
        }

        // Set the global collections at once after loading all data
        let _ = SCRIPTS.set(scripts);
        let _ = SCRIPT_NAMES.set(script_names);
        let _ = SCRIPT_LOOKUP.set(script_lookup);

        loaded
    }

    #[inline]
    pub fn get(id: usize) -> Option<ScriptFile> {
        let lookup = SCRIPT_LOOKUP.get()?;
        lookup.get(&id).cloned()
    }
    
    #[inline]
    pub fn get_by_trigger(trigger: ServerTriggerTypes, type_id: i32, category: i32) -> Option<ScriptFile> {
        Self::ensure_initialized();
        let lookup = SCRIPT_LOOKUP.get()?;

        if type_id != -1 {
            // Create key: trigger | (0x2 << 8) | (type_id << 10)
            let key = (trigger as u32 | (0x2 << 8) | ((type_id as u32) << 10)) as usize;
            if let Some(script) = lookup.get(&key) {
                return Some(script.clone());
            }
        }

        if category != -1 {
            // Create key: trigger | (0x1 << 8) | (category << 10)
            let key = (trigger as u32 | (0x1 << 8) | ((category as u32) << 10)) as usize;
            if let Some(script) = lookup.get(&key) {
                return Some(script.clone());
            }
        }

        // Fallback: return script for the trigger itself
        lookup.get(&(trigger as usize)).cloned()
    }

    #[inline]
    pub fn get_by_trigger_specific(trigger: ServerTriggerTypes, type_id: i32, category: i32) -> Option<ScriptFile> {
        Self::ensure_initialized();
        let lookup = SCRIPT_LOOKUP.get()?;

        // Early return pattern for clarity and performance
        if type_id != -1 {
            let key = (trigger as u32 | (0x2 << 8) | ((type_id as u32) << 10)) as usize;
            return lookup.get(&key).cloned();
        }

        if category != -1 {
            let key = (trigger as u32 | (0x1 << 8) | ((category as u32) << 10)) as usize;
            return lookup.get(&key).cloned();
        }

        // Fallback: return script for the trigger itself
        lookup.get(&(trigger as usize)).cloned()
    }
}