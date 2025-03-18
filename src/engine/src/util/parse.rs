use std::fs;
use std::path::Path;
use std::io::{Error, ErrorKind};
use crate::util::namemap::{list_dir, list_files};

pub fn read_text_normalize(path: &str) -> String {
    if !Path::new(path).exists() {
        return String::new();
    }
    match fs::read_to_string(path) {
        Ok(content) => content.replace("\r", ""),
        Err(_) => String::new(),
    }
}

// -----
// simple! just reads the file as-is
pub fn load_file(path: &str) -> Vec<String> {
    if !Path::new(path).exists() {
        return Vec::new();
    }
    read_text_normalize(path).split('\n').map(String::from).collect()
}

// fully-featured! strips out comments
pub fn load_file_full(path: &str) -> Result<Vec<String>, Error> {
    let text: Vec<String> = read_text_normalize(path).split('\n').map(String::from).collect();
    let mut lines = Vec::new();
    let mut multi_comment_start = 0;
    let mut multi_line_comments = 0;

    for (i, text_line) in text.iter().enumerate() {
        let mut line = text_line.trim().to_string();

        if multi_line_comments > 0 {
            let mut comment_start_idx = line.find("/*");
            while let Some(comment_start) = comment_start_idx {
                line = line[comment_start + 2..].trim_start().to_string();
                multi_line_comments += 1;
                comment_start_idx = line.find("/*");
            }

            let mut comment_end_idx = line.find("*/");
            while let Some(comment_end) = comment_end_idx {
                if multi_line_comments > 0 {
                    line = line[comment_end + 2..].trim_start().to_string();
                    multi_line_comments -= 1;
                    comment_end_idx = line.find("*/");
                } else {
                    break;
                }
            }

            if multi_line_comments > 0 {
                continue;
            }
        }

        if line.is_empty() {
            continue;
        }

        // if a line contains a single-line comment, strip it out
        if let Some(comment) = line.find("//") {
            line = line[..comment].trim_end().to_string();
            if line.is_empty() {
                continue;
            }
        }

        // if a line contains a multi-line comment, strip it out
        let comment_start = line.find("/*");
        let comment_end = line.find("*/");

        if let Some(start) = comment_start {
            if let Some(end) = comment_end {
                // comment ends on this line!
                line = line[..start].to_string() + &line[end + 2..];
            } else {
                // comment continues to another line
                line = line[..start].to_string();
                multi_line_comments += 1;
                if multi_comment_start == 0 {
                    multi_comment_start = i + 1;
                }
            }

            if line.is_empty() {
                continue;
            }
        }

        lines.push(line);
    }

    if multi_line_comments > 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} has an unclosed multi-line comment! Line: {}", path, multi_comment_start)
        ));
    }

    Ok(lines)
}

// ----
pub type LoadDirCallback = fn(lines: Vec<String>, file: &str);

// Read all files inside a directory
pub fn load_dir(path: &str, callback: LoadDirCallback) {
    let files = list_files(path);
    for file in files {
        let file_name = Path::new(&file)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        callback(load_file(&file), &file_name);
    }
}

// Read all files inside a directory with extra features
pub fn load_dir_full(path: &str, callback: LoadDirCallback) {
    let files = list_files(path);
    for file in files {
        let file_name = Path::new(&file)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if let Ok(lines) = load_file_full(&file) {
            callback(lines, &file_name);
        }
    }
}

// ----
// Generate a list of files inside a directory with a specific extension
pub fn list_files_ext(path: &str, ext: &str) -> Vec<String> {
    if !Path::new(path).exists() {
        return Vec::new();
    }
    list_dir(path)
        .into_iter()
        .filter(|x| x.ends_with(ext))
        .collect()
}

// Read all files inside a directory with a specific extension
pub fn load_dir_ext<F>(path: &str, ext: &str, mut callback: F)
where
    F: FnMut(Vec<String>, &str)
{
    let files = list_files_ext(path, ext);
    for file in files {
        callback(load_file(&file), &file);
    }
}

// Read all files inside a directory with a specific extension with extra features
pub fn load_dir_ext_full<F>(path: &str, ext: &str, mut callback: F)
where
    F: FnMut(Vec<String>, &str)
{
    let files = list_files_ext(path, ext);
    for file in &files {
        if let Ok(lines) = load_file_full(file) {
            callback(lines, file);
        }
    }
}

pub fn basename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map_or(String::new(), |name| name.to_string_lossy().to_string())
}

pub fn dirname(path: &str) -> String {
    Path::new(path)
        .parent()
        .map_or(String::new(), |p| p.to_string_lossy().to_string())
}