use std::collections::HashMap;
use log::{debug, error};
use rs2cache::Cache;
use rs2cache::js5_compression::Js5Compression;
use rs2cache::js5_index::Js5Index;
use crate::io::packet::Packet;
use crate::util::cache::param_helper::ParamValue;

pub fn unpack_objs() {
    let cache_path = "../../src/cacheLocal";

    let mut cache = Cache::open(cache_path).unwrap();
    let archive_id = 19; // Config archive
    let js5_index_compressed = cache.store.read(255, archive_id as u32).unwrap();
    let js5_index_decompressed = Js5Compression::uncompress(js5_index_compressed, None).unwrap();
    let js5_index = Js5Index::read(js5_index_decompressed).unwrap();

    let mut obj_count = 0;
    for i in 0..js5_index.groups.len() {
        for (j, _) in js5_index.groups.get(&(i as u32)).unwrap().files.iter() {
            obj_count += 1;
            debug!("Parsing obj: {}", obj_count);
            parse_obj(Packet::from(cache.read(19, i as u32, *j as u16, None).unwrap()));
        }
    }
    debug!("Parsed: {:?} 'obj' entries.", obj_count);
}

fn parse_obj(mut packet: Packet) {
    loop {
        let opcode = packet.g1();

        match opcode {
            0 => break,

            1 => {
                debug!("model: {}", packet.g2())
            },

            2 => {
                debug!("name: {}", packet.gjstr(0))
            },

            4 => {
                debug!("zoom2d: {}", packet.g2())
            }

            5 => {
                debug!("xan2d: {}", packet.g2())
            }

            6 => {
                debug!("yan2d: {}", packet.g2())
            }

            7 => {
                debug!("xof2d: {}", packet.g2());
            }

            8 => {
                debug!("yof2d: {}", packet.g2());
            }

            11 => {
                //debug!("stackable: true");
            }

            12 => {
                debug!("cost: {}", packet.g4());
            }

            16  => {
                //debug!("members: true");
            }

            23 => {
                debug!("manwear: {}", packet.g2())
            }

            24 => {
                debug!("manwear2: {}", packet.g2())
            }

            25 => {
                debug!("womanwear: {}", packet.g2());
            }

            26 => {
                debug!("womanwear: {}", packet.g2());
            }

            30 | 31 | 32 | 33 | 34 => {
                debug!("op{}: {}", opcode - 30, packet.gjstr(0));
            }

            35 | 36 | 37 | 38 | 39 => {
                debug!("iop{}: {}", opcode - 35, packet.gjstr(0));
            }

            40 => {
                let count = packet.g1();

                for i in 0..count {
                    packet.g2();
                    packet.g2();
                }
            }

            41 => {
                let count = packet.g1();

                for i in 0..count {
                    packet.g2();
                    packet.g2();
                }
            }

            42 => {
                let count = packet.g1();

                for i in 0..count {
                    packet.g1b();
                }
            }

            65 => {
                //debug!("getradeable: true");
            }

            78 => {
                debug!("manwear3: {}", packet.g2());
            }

            79 => {
                debug!("womanwear3: {}", packet.g2());
            }

            90 => {
                debug!("manhead: {}", packet.g2());
            }

            91 => {
                debug!("womanhead: {}", packet.g2());
            }

            92 => {
                debug!("manhead2: {}", packet.g2());
            }

            93 => {
                debug!("womanhead3: {}", packet.g2());
            }

            95 => {
                debug!("zan2d: {}", packet.g2());
            }

            96 => {
                debug!("dummyobject: {}", packet.g1());
            }

            97 => {
                debug!("certlink: {}", packet.g2());
            }
            
            98 => {
                debug!("certtemplate: {}", packet.g2());
            }
            
            100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 => {
                packet.g2();
                packet.g2();
            }
            
            110 => {
                debug!("resizex: {}", packet.g2());
            }
            
            111 => {
                debug!("resizey: {}", packet.g2());
            }
            
            112 => {
                debug!("resizez: {}", packet.g2());
            }
            
            113 => {
                debug!("ambient: {}", packet.g1b());
            }
            
            114 => {
                debug!("contrast: {}", packet.g1b() as i32 * 5);
            }
            
            115 => {
                debug!("team: {}", packet.g1());
            }
            
            121 => {
                debug!("lentlink: {}", packet.g2());
            }
            
            122 => {
                debug!("lenttemplate: {}", packet.g2());
            }
            
            125 | 126 => {
                packet.g1b();
                packet.g1b();
                packet.g1b();
            }
            
            127 | 128 | 129 | 130 => {
                packet.g1();
                packet.g2();
            }

            249 => {
                let count = packet.g1() as usize;

                let mut params: HashMap<i32, ParamValue> = HashMap::with_capacity(next_power_of_two(count));

                for _ in 0..count {
                    let is_string = packet.g1() == 1;
                    let key = packet.g3();

                    let value = if is_string {
                        ParamValue::String(packet.gjstr(0))
                    } else {
                        ParamValue::Integer(packet.g4())
                    };
                    
                    params.insert(key, value);
                }

                for (key, value) in params {
                    match value {
                        ParamValue::String(s) => debug!("params: {}: String(\"{}\")", key, s),
                        ParamValue::Integer(i) => debug!("params: {}: Integer({})", key, i),
                    }
                }

            }

            _ => {
                panic!("Unknown opcode {:?}", opcode);
            }
        }
    }
}

fn next_power_of_two(n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut power = 1;
    while power < n {
        power *= 2;
    }
    power
}