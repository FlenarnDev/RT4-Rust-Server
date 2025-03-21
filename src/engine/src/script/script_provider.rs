use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use log::{debug, error};
use tokio::io::AsyncReadExt;
use crate::io::packet::Packet;
use crate::script::script_file::ScriptFile;

// Maintains a list of scripts (id <-> name)
pub struct ScriptProvider {
    scripts: Vec<ScriptFile>,
    script_lookup: HashMap<u32, ScriptFile>,
    script_names: HashMap<String, u32>,
}

impl ScriptProvider {
    pub const COMPILER_VERSION: u32 = 23;
    
    pub fn load() -> u32 {
        println!("{}", Path::new("./data/pack/server/script.idx").exists());
        let dat = Packet::io("./data/pack/server/script.dat".parse().expect("ojoj")).expect("ojojojoj");
        let idx = Packet::io("./data/pack/server/script.idx".parse().unwrap()).unwrap();
        Self::parse(dat, idx)
    }
    
    fn parse(mut dat: Packet, mut idx: Packet) -> u32 {
        if dat.is_empty() || idx.is_empty() {
            error!("No scripts data found, rebuild scripts.");
            return 0;
        }
        
        let entries = dat.g4();
        debug!("Script entries found: {:?}", entries);
        
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

                // add the script to lookup table if the value isn't -1
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
        
        loaded
    }
}