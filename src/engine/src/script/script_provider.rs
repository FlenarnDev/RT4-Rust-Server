use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use log::{debug, error};
use crate::io::packet::Packet;
use crate::script::script_file::ScriptFile;
use crate::script::server_trigger_types::ServerTriggerTypes;

static SCRIPTS: OnceLock<Vec<ScriptFile>> = OnceLock::new();
static SCRIPT_LOOKUP: OnceLock<HashMap<usize, ScriptFile>> = OnceLock::new();
static SCRIPT_NAMES: OnceLock<HashMap<String, usize>> = OnceLock::new();

pub struct ScriptProvider;

impl ScriptProvider {
    pub const COMPILER_VERSION: u32 = 23;
    
    fn ensure_initialized() {
        if SCRIPTS.get().is_none() {
            SCRIPTS.set(Vec::new()).unwrap();
            SCRIPT_LOOKUP.set(HashMap::new()).unwrap();
            SCRIPT_NAMES.set(HashMap::new()).unwrap();
        }
    }
    
    pub fn load() -> u32 {
        let dat_path = "./data/pack/server/script.dat".to_string();
        let idx_path = "./data/pack/server/script.idx".to_string();
        
        let dat = Packet::io(dat_path).unwrap();
        let idx = Packet::io(idx_path).unwrap();
        let count = Self::parse(dat, idx);
        debug!("Loaded {:?} scripts", count);
        count
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
        
        let mut scripts: Vec<ScriptFile> = Vec::with_capacity(entries as usize);
        let mut script_names: HashMap<String, usize> = HashMap::new();
        let mut script_lookup: HashMap<usize, ScriptFile> = HashMap::new();

        let mut loaded = 0;
        for id in 0..entries {
            let size = idx.g4();
            if size == 0 {
                continue;
            }

            match (|| {
                let script = ScriptFile::decode(id as usize, Packet::from(dat.gbytes(size as usize)));
                scripts.push(script.clone());
                script_names.insert(script.info.script_name.clone(), id as usize);

                if script.info.lookup_key != -1 {
                    script_lookup.insert(script.info.lookup_key as usize, script);
                }
                
                loaded += 1;
                Ok::<_, Box<dyn Error>>(())
            })() {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("{}", err);
                    eprintln!("Warning: Failed to load script {}, something may have been partially written", id);
                    return 0;
                }
            }
        }
        
        SCRIPTS.set(scripts).unwrap();
        SCRIPT_NAMES.set(script_names).unwrap();
        SCRIPT_LOOKUP.set(script_lookup).unwrap();
        
        loaded
    }
    
    pub fn get_by_trigger(trigger: ServerTriggerTypes, type_id: i32, category: i32) -> Option<ScriptFile> {
        Self::ensure_initialized();
        let lookup = SCRIPT_LOOKUP.get()?;
        
        if type_id == -1 {
            let key: usize = (trigger as u32 | (0x2 << 8) | ((type_id as u32) << 10)) as usize;
            if let Some(script) = lookup.get(&key) {
                return Some(script.clone());
            }
        }
        
        if category == -1 {
            let key: usize = (trigger as u32 | (0x1 << 8) | ((type_id as u32) << 10)) as usize;
            if let Some(script) = lookup.get(&key) {
                return Some(script.clone());
            }
        }
        
        lookup.get(&(trigger as usize)).cloned()
    }
    
    pub fn get_by_trigger_specific(trigger: ServerTriggerTypes, type_id: i32, category: i32) -> Option<ScriptFile> {
        Self::ensure_initialized();
        let lookup = SCRIPT_LOOKUP.get()?;
        
        if trigger as i32 != -1 {
            let key: usize = (trigger as u32 | (0x2 << 8) | ((type_id as u32) << 10)) as usize;
            return lookup.get(&key ).cloned();
        } else if category != -1 {
            let key: usize = (trigger as u32 | (0x1 << 8) | ((category as u32) << 10)) as usize;
            return lookup.get(&key).cloned()
        } 
        
        lookup.get(&(trigger as usize)).cloned()
    }
}