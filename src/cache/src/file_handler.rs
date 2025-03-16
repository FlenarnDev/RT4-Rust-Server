use std::collections::HashMap;
use std::sync::RwLock;
use std::error;
use std::time::Instant;
use once_cell::sync::Lazy;
use log::{debug, error, info};
use rs2cache::Cache;
use rs2cache::js5_compression::Js5Compression;
use rs2cache::js5_index::Js5Index;
use rs2cache::js5_masterindex::Js5MasterIndex;
use rs2cache::store::ARCHIVESET;

struct CacheData {
    preloaded_data: HashMap<(u8, u16), Vec<u8>>,
    master_index: Option<Vec<u8>>,
    checksums: Vec<u32>,
    cache_path: String
}

static GLOBAL_CACHE_DATA: Lazy<RwLock<CacheData>> = Lazy::new(|| {
    RwLock::new(CacheData {
        preloaded_data: HashMap::with_capacity(67551),
        master_index: None,
        checksums: Vec::new(),
        cache_path: "../../src/cacheLocal".to_string()
    })
});

static INIT: Lazy<()> = Lazy::new(|| {
    info!("Initializing global cache");
    match initialize_cache() {
        Ok(_) => info!("Cache initialized successfully"),
        Err(e) => error!("Failed to initialize cache: {}", e),
    }
});

fn initialize_cache() -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    let cache_path = "../../src/cacheLocal";

    let cache = match Cache::open(cache_path) {
        Ok(cache) => cache,
        Err(e) => {
            return Err(format!("Failed to open cache: {}", e).into());
        }
    };

    let master_index = Js5MasterIndex::create(&cache.store);
    let master_index_data = master_index.write();

    let mut checksums: Vec<u32> = Vec::with_capacity(master_index.entries.len());

    let mut preloaded_data = HashMap::with_capacity(67553);
    let mut total_entries = 0;
    let mut successful_loads = 0;
    let mut failed_loads = 0;
    
    for archive_id in 0..master_index.entries.len() {
        checksums.push(master_index.entries[archive_id].checksum);
        let js5_index_compressed = cache.store.read(255, archive_id as u32).unwrap();
        let js5_index_decompressed = Js5Compression::uncompress(js5_index_compressed, None)?;
        let js5_index = Js5Index::read(js5_index_decompressed).unwrap();
        for (group_id, _) in js5_index.groups.iter() {
            match cache.store.read(archive_id as u8, *group_id) {
                Ok(data) => {
                    preloaded_data.insert((archive_id as u8, *group_id as u16), data);
                    successful_loads += 1;
                },
                Err(e) => {
                    error!("Archive: {}, group: {} : {}",archive_id, group_id, e);
                    failed_loads += 1;
                }
            }
            total_entries += 1
        }
    }

    // Handle the master index data separately.
    let master_index_entries_len = master_index.entries.len();
    for group_id in 0..master_index_entries_len {
        match cache.store.read(ARCHIVESET, group_id as u32) {
            Ok(data) => {
                preloaded_data.insert((ARCHIVESET, group_id as u16), data);
            },
            Err(e) => {
                error!("Archive: {}, group: {} : {}", ARCHIVESET, group_id, e);
                failed_loads += 1;
            }
        }
    }

    info!("Preloaded cache in {:?}", start.elapsed());
    info!(
        "Preloaded {}/{} cache entries ({:.3}%)",
        successful_loads,
        total_entries,
        (successful_loads as f64 / total_entries as f64) * 100.0
    );
    info!(
        "Failed to preload {}/{} cache entries ({:.3}%)",
        failed_loads,
        total_entries,
        (failed_loads as f64 / total_entries as f64) * 100.0
    );

    let mut global_data = GLOBAL_CACHE_DATA.write().unwrap();
    global_data.preloaded_data = preloaded_data;
    global_data.master_index = Some(master_index_data);
    global_data.checksums = checksums;
    global_data.cache_path = cache_path.to_string();

    Ok(())
}


pub fn ensure_initialized() -> Result<(), Box<dyn error::Error>> {
    Lazy::force(&INIT);
    Ok(())
}

pub fn get_data(archive: u8, group: u16) -> Result<Vec<u8>, Box<dyn error::Error>> {
    ensure_initialized()?;
    
    {
        let data_cache = GLOBAL_CACHE_DATA.read().unwrap();
        if let Some(data) = data_cache.preloaded_data.get(&(archive, group)) {
            return Ok(data.clone());
        }
    }

    // This should never occur, but here for safety.
    let cache_path = {
        let data_cache = GLOBAL_CACHE_DATA.read().unwrap();
        data_cache.cache_path.clone()
    };

    debug!("Data for archive {}, group {} not in preloaded cache, loading directly", archive, group);

    let cache = Cache::open(&cache_path)?;
    let data = cache.store.read(archive, group as u32)?;

    let mut preloaded_data = GLOBAL_CACHE_DATA.write().unwrap();
    preloaded_data.preloaded_data.insert((archive, group), data.clone());

    Ok(data)
}

pub fn get_master_index() -> Result<Vec<u8>, Box<dyn error::Error>> {
    ensure_initialized()?;
    let data_cache = GLOBAL_CACHE_DATA.read().unwrap();
    data_cache.master_index.clone()
        .ok_or_else(|| "Master index not initialized".into())
}

pub fn get_checksum(archive_id: usize) -> Result<u32, Box<dyn error::Error>> {
    ensure_initialized()?;
    let data_cache = GLOBAL_CACHE_DATA.read().unwrap();
    data_cache.checksums.get(archive_id)
        .copied()
        .ok_or_else(|| format!("Checksum not found for archive: {}", archive_id).into())
}