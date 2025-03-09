use std::sync::OnceLock;
use log::{debug, error};
use rs2cache::Cache;
use rs2cache::js5_masterindex::Js5MasterIndex;

pub static CACHE: OnceLock<Cache> = OnceLock::new();
pub static MASTER_INDEX_VEC: OnceLock<Vec<u8>> = OnceLock::new();

pub fn init() {
    debug!("Initializing cacheLocal from ./cacheLocal");
    let cache = match Cache::open("../../src/cacheLocal") {
        Ok(cache) => {
            debug!("Archive count: {:?}", cache.archives.len());
            cache
        },
        Err(e) => {
            error!("Error opening cache: {}", e);
            return
        }
    };
    let master_index_vec = Js5MasterIndex::create(&cache.store).write();

    if CACHE.set(cache).is_err() {
        error!("Failed to set cache in global storage");
        return;
    }
    
    if MASTER_INDEX_VEC.set(master_index_vec).is_err() {
        error!("Failed to set master index vector in global storage");
    }
}