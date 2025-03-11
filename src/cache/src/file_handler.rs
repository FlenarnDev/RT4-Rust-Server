use std::fs;
use std::path::Path;
use std::cell::RefCell;
use std::sync::Once;
use log::{debug, error};
use rs2cache::Cache;
use rs2cache::js5_masterindex::Js5MasterIndex;

// Using thread_local! with lazy initialization pattern
thread_local! {
    static CACHE_INITIALIZED: RefCell<bool> = RefCell::new(false);
    pub static CACHE: RefCell<Option<Cache>> = RefCell::new(None);
    pub static MASTER_INDEX_VEC: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}

// Initialize the cache in the current thread if it hasn't been initialized yet
pub fn ensure_initialized() -> Result<(), Box<dyn std::error::Error>> {
    let mut needs_init = false;

    CACHE_INITIALIZED.with(|initialized| {
        if !*initialized.borrow() {
            *initialized.borrow_mut() = true;
            needs_init = true;
        }
    });

    if needs_init {
        debug!("Initializing cache in thread: {:?}", std::thread::current().id());

        // Open the cache
        let cache = match Cache::open("../../src/cacheLocal") {
            Ok(cache) => cache,
            Err(e) => {
                error!("Error opening cache: {}", e);
                return Err(format!("Failed to open cache: {}", e).into());
            }
        };

        // Generate the master index
        let master_index_vec = Js5MasterIndex::create(&cache.store).write();

        // Store in thread-local storage
        CACHE.with(|cache_ref| {
            *cache_ref.borrow_mut() = Some(cache);
        });

        MASTER_INDEX_VEC.with(|master_vec_ref| {
            *master_vec_ref.borrow_mut() = Some(master_index_vec);
        });

        debug!("Cache initialized successfully in thread: {:?}", std::thread::current().id());
    }

    Ok(())
}