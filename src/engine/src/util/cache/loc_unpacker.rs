use std::fs::File;
use log::{debug};
use rs2cache::Cache;
use rs2cache::js5_compression::Js5Compression;
use rs2cache::js5_index::Js5Index;
use constants::js5_archive::js5_archive::CONFIG_LOC;
use crate::io::packet::Packet;
use crate::util::cache::config::config_type::ConfigType;
use crate::util::cache::config::loc_type::{write_loc, LocType};

pub fn unpack_locs() {
    let cache_path = "../../src/cacheLocal";
    
    let mut cache = Cache::open(cache_path).unwrap();
    let archive_id = CONFIG_LOC;
    let js5_index_compressed = cache.store.read(255, archive_id).unwrap();
    let js5_index_decompressed = Js5Compression::uncompress(js5_index_compressed, None).unwrap();
    let js5_index = Js5Index::read(js5_index_decompressed).unwrap();
    
    let mut file = File::create("data/src/scripts/_unpack/all.loc").unwrap();
    
    let mut loc_count = 0;
    for i in 0..js5_index.groups.len() {
        for (j, _) in js5_index.groups.get(&(i as u32)).unwrap().files.iter() {
            let mut loc = LocType::new(loc_count);
            let mut opcode_order: Vec<u8> = Vec::new();
            loc.decode_type(&mut Packet::from(cache.read(CONFIG_LOC as u8, i as u32, *j as u16, None).unwrap()), &mut opcode_order);

            loc_count += 1;
            write_loc(&mut file, &mut loc, &mut opcode_order);
        }
    }
    debug!("Parsed: {:?} 'loc' entries.", loc_count + 1);
}