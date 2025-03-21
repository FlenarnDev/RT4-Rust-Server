use std::collections::HashMap;
use std::path::Path;
use log::{debug, error};
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
    
    fn parse(mut dat: Packet, idx: Packet) -> u32 {
        if dat.is_empty() || idx.is_empty() {
            error!("No scripts data found, rebuild scripts.");
            return 0;
        }
        
        debug!("dat size: {:?}", dat.len());
        
        let entries = dat.g4();
        debug!("Entries: {:?}", entries);
        entries as u32
    }
}