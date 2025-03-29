use std::fs::File;
use log::{debug};
use rs2cache::Cache;
use rs2cache::js5_compression::Js5Compression;
use rs2cache::js5_index::Js5Index;
use crate::io::packet::Packet;
use crate::util::cache::config::config_type::ConfigType;
use crate::util::cache::config::obj_type::{write_obj, ObjType};

pub fn unpack_objs() {
    let cache_path = "../../src/cacheLocal";

    let mut cache = Cache::open(cache_path).unwrap();
    let archive_id = 19; // Config archive
    let js5_index_compressed = cache.store.read(255, archive_id as u32).unwrap();
    let js5_index_decompressed = Js5Compression::uncompress(js5_index_compressed, None).unwrap();
    let js5_index = Js5Index::read(js5_index_decompressed).unwrap();

    let mut file = File::create("../../../data/src/scripts/_unpack/all.obj").unwrap();
    
    let mut obj_count = 0;
    for i in 0..js5_index.groups.len() {
        for (j, _) in js5_index.groups.get(&(i as u32)).unwrap().files.iter() {
            let mut obj = ObjType::new(obj_count);
            let mut opcode_order: Vec<u8> = Vec::new();
            obj.decode_type(&mut Packet::from(cache.read(19, i as u32, *j as u16, None).unwrap()), &mut opcode_order);
            
            obj_count += 1;
            write_obj(&mut file, &mut obj, &mut opcode_order);
        }
    }
    debug!("Parsed: {:?} 'obj' entries.", obj_count + 1);
}