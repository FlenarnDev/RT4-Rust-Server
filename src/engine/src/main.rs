use std::path::Path;
use log::debug;
use engine::engine::Engine;
use engine::util::pack_file::revalidate_pack;
use engine::util::symbols::generate_server_symbols;

fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    if !Path::new("./data").exists() {
        
        debug!("No such file or directory");
    }

    revalidate_pack();
    generate_server_symbols();
    
    //Engine::new().start(true);
}