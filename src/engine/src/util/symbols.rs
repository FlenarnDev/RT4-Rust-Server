use log::debug;
use std::fs;use crate::util::namemap::load_pack;

pub fn generate_server_symbols() {
    fs::create_dir_all("../../data/symbols").expect("Failed to create symbols directory");
    
    let scripts = load_pack("./data/src/pack/script.pack");
    let mut script_symbols = String::new();
    debug!("{:?}", scripts.len());

    for (i, script) in scripts.iter().enumerate() {
        if !script.is_empty() {
            script_symbols.push_str(&format!("{}\t{}\n", i, script));
        }
    }

    fs::write("../../data/symbols/runescript.sym", script_symbols).expect("Failed to write to symbols file");
}