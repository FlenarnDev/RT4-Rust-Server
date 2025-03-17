use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use std::time::Instant;
use std::error;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct XTEAKey(pub i32, pub i32, pub i32, pub i32);

impl XTEAKey {
    pub const ZERO: Self = XTEAKey(0, 0, 0, 0);

    /// Check if XTEA key is equal to: {0, 0, 0, 0}
    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0
    }

    /// Convert to array
    pub fn to_array(&self) -> [i32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct XTEAData {
    archive: i32,
    group: i32,
    #[serde(rename= "name_hash")]
    name_hash: i32,
    name: String,
    mapsquare: i32,
    key: XTEAKey,
}

static XTEA_MAP: OnceLock<HashMap<i32, XTEAKey>> = OnceLock::new();
pub fn initialize_xtea() -> Result<bool, Box<dyn error::Error>> {
    let start = Instant::now();
    info!("Initializing XTEA module.");
    
    let mut file = File::open("../../src/cacheLocal/xteaKeys.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let keys_list: Vec<XTEAData> = serde_json::from_str(&contents)?;
    
    let mut map = HashMap::new();
    
    for data in keys_list {
        let xtea_key = XTEAKey(data.key.0, data.key.1, data.key.2, data.key.3);
        map.insert(data.mapsquare, xtea_key);
    }

    if let Err(_) = XTEA_MAP.set(map) {
        error!("XTEA map was already initialized");
    }

    info!("XTEA count: {}", XTEA_MAP.get().unwrap().len());
    info!("XTEA Module initiated in: {}ms.", start.elapsed().as_millis());

    Ok(true)
}

pub fn get_xtea_key_by_mapsquare(mapsquare: i32) -> XTEAKey {
    if let Some(map) = XTEA_MAP.get() {
        map.get(&mapsquare).copied().unwrap_or(XTEAKey::ZERO)
    } else {
        XTEAKey::ZERO
    }
}