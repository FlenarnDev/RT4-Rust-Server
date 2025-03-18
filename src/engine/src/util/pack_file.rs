use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::debug;
use crate::util::parse::{load_dir_ext_full, load_file};

#[derive(Clone)]
pub struct ValidatorArg;

type PackFileValidator = fn(&mut PackFile, &[ValidatorArg]) -> ();

pub struct PackFile {
    pub type_name: String,
    validator: Option<PackFileValidator>,
    validator_args: Vec<ValidatorArg>,
    pack: HashMap<u32, String>,
    names: HashSet<String>,
    max: u32,
}

impl PackFile {
    pub fn new(type_name: String, validator: Option<PackFileValidator>, validator_args: Vec<ValidatorArg>) -> Self {
        let mut pack_file = Self {
            type_name,
            validator,
            validator_args,
            pack: HashMap::new(),
            names: HashSet::new(),
            max: 0,
        };
        pack_file.reload();
        pack_file
    }
    
    pub fn reload(&mut self) {
        if let Some(validator) = self.validator {
            let args_clone = self.validator_args.clone();
            validator(self, &args_clone);
        } else {
            // TODO - environment specification goes here.
            let path = format!("./data/src/pack/{}.pack", self.type_name);
            self.load(&path);
        }
    }
    
    pub fn load(&mut self, path: &str) {
        self.pack = HashMap::new();
        
        if !Path::new(path).exists() {
            return;
        }
        
        let lines = load_file(path);
        
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() || !line.starts_with(|c: char| c.is_digit(10)) || !line.contains('=') {
                continue;
            }
            
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() < 2 || parts[1].is_empty() {
                panic!("Pack file has empty name {}:{}", path, i + 1);
            }
            
            let id = parts[0].parse::<u32>().unwrap();
            self.register(id, parts[1].to_string());
        }
        self.refresh_names();
    }
    
    pub fn register(&mut self, id: u32, name: String) {
        self.pack.insert(id, name);
    }
    
    pub fn refresh_names(&mut self) {
        self.names = self.pack.values().cloned().collect();
        self.max = self.pack.keys().max().map_or(0, |&max| max + 1);
    }
    
    pub fn save(&self) {
        let mut entries: Vec<(&u32, &String)> = self.pack.iter().collect();
        entries.sort_by_key(|&(id, _)| id);
        
        let content = entries
            .iter()
            .map(|(id, name)| format!("{}={}", id, name))
            .collect::<Vec<String>>()
            .join("\n") + "\n";
        
        fs::write(
            format!("{}/pack/{}.pack", "./data/src", self.type_name),
            content,
        ).expect("Unable to write pack file");
    }
    
    pub fn get_by_id(&self, id: u32) -> String {
        self.pack.get(&id).cloned().unwrap_or_default()
    }
    
    pub fn get_by_name(&self, name: &str) -> i32 {
        if !self.names.contains(name) {
            return -1;
        }
        
        for (id, pack_name) in &self.pack {
            if pack_name == name {
                return *id as i32;
            }
        }
        
        // No match found.
        -1
    }
}

fn regenerate_script_pack(pack: &mut PackFile, _args: &[ValidatorArg]) {
    debug!("regenerate_script_pack");
    let path = format!("{}/pack/script.pack", "./data/src");
    pack.load(&path);
    
    let names = crawl_config_names(".rs2", true);
    for name in names {
        if !pack.names.contains(&name) {
            let max = pack.max;
            pack.register(max, name);
            pack.max += 1;
        }
    }
    
    pack.refresh_names();
    pack.save();
}


lazy_static! {
    pub static ref SCRIPT_PACK: Mutex<PackFile> = {
        let empty_validator_args: Vec<ValidatorArg> = Vec::new();
        Mutex::new(PackFile::new("script".to_string(), Some(regenerate_script_pack), empty_validator_args))
    };
}

pub fn revalidate_pack() {
    SCRIPT_PACK.lock().unwrap().reload();
}

pub fn crawl_config_names(ext: &str, include_brackets: bool) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let script_path = format!("{}/scripts", "./data/src");

    load_dir_ext_full(&script_path, ext, |lines, file| {
        // Skip engine.rs2 file
        if file == format!("{}/scripts/engine.rs2", "./data/src") {
            return;
        }

        for line in lines {
            if !line.starts_with("[") {
                continue;
            }

            if let Some(closing_bracket_idx) = line.find("]") {
                let mut name = line[0..closing_bracket_idx + 1].to_string();

                if !include_brackets {
                    name = line[1..closing_bracket_idx].to_string();
                }

                // Verify folders
                let file_path = PathBuf::from(file);
                let file_dir = file_path.parent()
                    .and_then(|p| p.file_name())
                    .map_or(String::new(), |n| n.to_string_lossy().to_string());

                let parent_dir = file_path.parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.file_name())
                    .map_or(String::new(), |n| n.to_string_lossy().to_string());

                // Skip directory verification for _unpack or .flo files
                if file_dir != "_unpack" && ext != ".flo" {
                    // Verify scripts are in a scripts directory
                    if ext == ".rs2" && file_dir != "scripts" && parent_dir != "scripts" {
                        panic!("Script file {} must be located inside a \"scripts\" directory.", file);
                    }
                    // Verify configs are in a configs directory
                    else if ext != ".rs2" && file_dir != "configs" && parent_dir != "configs" {
                        panic!("Config file {} must be located inside a \"configs\" directory.", file);
                    }
                }

                // Add name if it's not already in the list
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }
    });

    names
}