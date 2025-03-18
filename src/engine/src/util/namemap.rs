use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

// cached directory listings
lazy_static! {
    static ref DIR_CACHE: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn list_dir(path_str: &str) -> Vec<String> {
    let mut path = path_str.to_string();
    if path.ends_with('/') {
        path.truncate(path.len() - 1);
    }

    // Try to get from cache first
    let cache_result = {
        let cache = DIR_CACHE.lock().unwrap();
        cache.get(&path).cloned()
    };

    if let Some(files) = cache_result {
        let mut all: Vec<String> = Vec::new();
        for file in &files {
            let full_path = format!("{}/{}", path, file);
            all.push(full_path.clone());
            if file.ends_with('/') {
                all.extend(list_dir(&full_path));
            }
        }
        return all;
    }

    // No cached result, read from filesystem
    if !Path::new(&path).exists() {
        return Vec::new();
    }

    let read_dir_result = fs::read_dir(&path);
    if read_dir_result.is_err() {
        return Vec::new();
    }

    let mut files: Vec<String> = Vec::new();

    for entry_result in read_dir_result.unwrap() {
        if let Ok(entry) = entry_result {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = format!("{}/{}", path, file_name);

            if let Ok(metadata) = fs::metadata(&file_path) {
                if metadata.is_dir() {
                    files.push(format!("{}/", file_name));
                } else {
                    files.push(file_name);
                }
            }
        }
    }

    // Cache the results
    {
        let mut cache = DIR_CACHE.lock().unwrap();
        cache.insert(path.clone(), files.clone());
    }

    // Build the full list
    let mut all: Vec<String> = Vec::new();
    for file in &files {
        let full_path = format!("{}/{}", path, file);
        all.push(full_path.clone());
        if file.ends_with('/') {
            all.extend(list_dir(&full_path));
        }
    }

    all
}

pub fn load_order(path: &str) -> Vec<i32> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    match fs::read_to_string(path) {
        Ok(content) => content
            .replace("\r", "")
            .split('\n')
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.parse::<i32>().ok())
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn load_pack(path: &str) -> Vec<String> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    match fs::read_to_string(path) {
        Ok(content) => {
            let mut result: Vec<String> = Vec::new();

            for line in content.replace("\r", "").split('\n').filter(|x| !x.is_empty()) {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() >= 2 {
                    if let Ok(id) = parts[0].parse::<usize>() {
                        // Ensure the vector is large enough
                        while result.len() <= id {
                            result.push(String::new());
                        }
                        result[id] = parts[1].to_string();
                    }
                }
            }

            result
        },
        Err(_) => Vec::new(),
    }
}

pub type LoadDirCallback = fn(src: Vec<String>, file: &str, path: &str);

pub fn load_dir(path: &str, extension: &str, callback: LoadDirCallback) {
    let files = list_dir(path);

    for file in files {
        if file.ends_with(extension) {
            let path_buf = PathBuf::from(&file);
            let file_name = path_buf.file_name().map_or(String::new(), |n| n.to_string_lossy().to_string());
            let dir_path = path_buf.parent().map_or(String::new(), |p| p.to_string_lossy().to_string());

            if let Ok(content) = fs::read_to_string(&file) {
                let lines: Vec<String> = content
                    .replace("\r", "")
                    .split('\n')
                    .filter(|x| !x.is_empty())
                    .map(String::from)
                    .collect();

                callback(lines, &file_name, &dir_path);
            }
        }
    }
}

pub fn load_dir_exact(path: &str, extension: &str, callback: LoadDirCallback) {
    let files = list_dir(path);

    for file in files {
        if file.ends_with(extension) {
            let path_buf = PathBuf::from(&file);
            let file_name = path_buf.file_name().map_or(String::new(), |n| n.to_string_lossy().to_string());
            let dir_path = path_buf.parent().map_or(String::new(), |p| p.to_string_lossy().to_string());

            if let Ok(content) = fs::read_to_string(&file) {
                let lines: Vec<String> = content
                    .replace("\r", "")
                    .split('\n')
                    .map(String::from)
                    .collect();

                callback(lines, &file_name, &dir_path);
            }
        }
    }
}

pub fn list_files(path: &str) -> Vec<String> {
    list_dir(path)
}