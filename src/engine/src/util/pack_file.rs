use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Deref;
use std::path::Path;
use crate::util::parse::load_file;

type PackFileValidator = fn(&PackFile, &[Box<dyn std::any::Any>]) -> ();

pub struct PackFile {
    pub type_name: String,
    validator: Option<PackFileValidator>,
    validator_args: Vec<Box<dyn std::any::Any>>,
    pack: HashMap<u32, String>,
    names: HashSet<String>,
    max: u32,
}

impl PackFile {
    pub fn new(type_name: String, validator: Option<PackFileValidator>, validator_args: Vec<Box<dyn std::any::Any>>) -> Self {
        let mut pack_file = Self {
            type_name,
            validator,
            validator_args,
            pack: HashMap::new(),
            names: HashSet::new(),
            max: 0,
        };
        
        pack_file
    }
    
    pub fn size(&self) -> usize {
        self.pack.len()
    }
    
    pub fn reload(&mut self) {
        if let Some(validator) = self.validator {
            validator(self, &self.validator_args);
        } else {
            // TODO - environment specification goes here.
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
            format!("{}/pack/{}.pack", "./", self.type_name),
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