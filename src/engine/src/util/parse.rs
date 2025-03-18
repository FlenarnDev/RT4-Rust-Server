use std::fs;
use std::path::Path;

/// Reads a text file and normalizes line endings by removing carriage returns.
pub fn read_text_normalize(path: &str) -> String {
    if !Path::new(path).exists() {
        return String::new();
    }
    
    match fs::read_to_string(path) { 
        Ok(content) => content.replace("\r", ""),
        Err(_) => String::new(),
    }
}

/// Simple function that reads a file as-is and splits it into lines.
pub fn load_file(path: &str) -> Vec<String> {
    if !Path::new(path).exists() {
        return Vec::new();
    }   
    
    read_text_normalize(path)
        .split('\n')
        .map(|s| s.to_string())
        .collect()
}