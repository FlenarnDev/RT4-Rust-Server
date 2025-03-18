use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use log::{debug, info};
use reqwest::blocking;
use sha2::Digest;
use crate::script::script_provider::ScriptProvider;

pub fn update_compiler() -> Result<bool, Box<dyn Error>> {
    info!("Checking for compiler update.");
    let mut needs_update = false;

    if !Path::new("./RuneScriptCompiler.jar").exists() {
       needs_update = true;
    } else {
        let mut file = File::open("./RuneScriptCompiler.jar")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = sha2::Sha256::new();
        hasher.update(&buffer);
        let shasum = format!("{:x}", hasher.finalize());

        let sha_url = format!(
            "https://github.com/LostCityRS/RuneScriptKt/releases/download/{}/RuneScriptCompiler.jar.sha256",
            ScriptProvider::COMPILER_VERSION
        );

        let sha_response = blocking::get(&sha_url)?;
        let expected = sha_response.text()?[..64].to_string();

        if shasum != expected {
            needs_update = true;
        }
    }

    if needs_update {
        info!("Updating compiler.");
        let jar_url = format!(
            "https://github.com/LostCityRS/RuneScriptKt/releases/download/{}/RuneScriptCompiler.jar",
            ScriptProvider::COMPILER_VERSION
        );

        let jar_response = blocking::get(&jar_url)?;
        let jar_bytes = jar_response.bytes()?;

        fs::write("RuneScriptCompiler.jar", jar_bytes)?;
    }

    info!("Compiler is up to date.");
    Ok(true)
}