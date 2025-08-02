use std::fs::File;
use crate::io::packet::Packet;
use crate::util::cache::config::config_type::ConfigType;
use crate::util::cache::param_helper::{decode_params, ParamValue, Params};
use std::io::Write;
use log::{debug, error};

#[derive(Debug)]
pub struct ObjType {
    pub id: u32,
    debugname: Option<String>,
    model: u32,
    pub name: Option<String>,
    recol_s: Vec<u16>,
    recol_d: Vec<u16>,
    retex_s: Vec<u16>,
    retex_d: Vec<u16>,
    recol_d_palette: Vec<i8>,
    zoom2d: u32,
    xan2d: u32,
    yan2d: u32,
    xof2d: i32,
    yof2d: i32,
    stackable: bool,
    cost: i32,
    members: bool,
    manWear: i32,
    manWear2: i32,
    manWear3: i32,
    womanWear: i32,
    womanWear2: i32,
    womanWear3: i32,
    op: Vec<String>,
    iop: Vec<String>,
    stockmarket_yes: bool,
    manHead: i32,
    manHead2: i32,
    womanHead: i32,
    womanHead2: i32,
    zand2d: u32,
    dummyItem: u8,
    certlink: i32,
    certtemplate: i32,
    countobj: Option<Vec<u16>>,
    countco: Option<Vec<u16>>,
    resizex: u32,
    resizey: u32,
    resizez: u32,
    ambient: i8,
    contrast: i32,
    team: u8,
    lentlink: i32,
    lenttemplate: i32,
    manwearxoffset: i8,
    manwearyoffset: i8,
    manwearzoffset: i8,
    womanwearxoffset: i8,
    womanwearyoffset: i8,
    womanwearzoffset: i8,
    cursor1op: i8,
    cursor1: i32,
    cursor2op: i8,
    cursor2: i32,
    params: Params,
}

impl ObjType {
    pub fn new(id: u32) -> Self {
        ObjType {
            id,
            debugname: None,
            model: 0,
            name: None,
            recol_s: Vec::new(),
            recol_d: Vec::new(),
            retex_s: Vec::new(),
            retex_d: Vec::new(),
            recol_d_palette: Vec::new(),
            zoom2d: 2000,
            xan2d: 0,
            yan2d: 0,
            xof2d: 0,
            yof2d: 0,
            stackable: false,
            cost : 1,
            members: false,
            manWear: -1,
            manWear2: -1,
            manWear3: -1,
            womanWear: -1,
            womanWear2: -1,
            womanWear3: -1,
            op: vec!["".to_string(); 5],
            iop: vec!["".to_string(); 5],
            stockmarket_yes: false,
            manHead: -1,
            manHead2: -1,
            womanHead: -1,
            womanHead2: -1,
            zand2d: 0,
            dummyItem: 0,
            certlink: -1,
            certtemplate: -1,
            countobj: None,
            countco: None,
            resizex: 128,
            resizey: 128,
            resizez: 128,
            ambient: 0,
            contrast: 0,
            team: 0,
            lentlink: -1,
            lenttemplate: -1,
            manwearxoffset: 0,
            manwearyoffset: 0,
            manwearzoffset: 0,
            womanwearxoffset: 0,
            womanwearyoffset: 0,
            womanwearzoffset: 0,
            cursor1op: -1,
            cursor1: -1,
            cursor2op: -1,
            cursor2: -1,
            params: Params::default(),
        }
    }
}

impl ConfigType for ObjType {
    fn id(&self) -> u32 {
        self.id
    }

    fn debugname(&self) -> Option<&String> {
        self.debugname.as_ref()
    }

    fn set_debugname(&mut self, debugname: String) {
        self.debugname = Some(debugname);
    }

    fn decode(&mut self, opcode: u8, packet: &mut Packet) {
        match opcode {
            1 => {
                self.model = packet.g2() as u32;
            }
            
            2 => {
                self.name = Some(packet.gjstr());
            }
            
            4 => {
                self.zoom2d = packet.g2() as u32;
            }
            
            5 => {
                self.xan2d = packet.g2() as u32;
            }
            
            6 => {
                self.yan2d = packet.g2() as u32;
            }
            
            7 => {
                let mut xof2d = packet.g2() as i32;
                
                if xof2d > 32767 {
                    xof2d -= 65536;
                }
                self.xof2d = xof2d;
            }
            
            8 => {
                let mut yof2d = packet.g2() as i32;

                if yof2d > 32767 {
                    yof2d -= 65536;
                }
                self.yof2d = yof2d;
            }
            
            11 => {
                self.stackable = true;
            }
            
            12 => {
                self.cost = packet.g4();
            }
            
            16 => {
                self.members = true;
            }
            
            23 => {
                self.manWear = packet.g2() as i32;
            }
            
            24 => {
                self.manWear2 = packet.g2() as i32;
            }
            
            25 => {
                self.womanWear = packet.g2() as i32;
            }
            
            26 => {
                self.womanWear2 = packet.g2() as i32;
            }
            
            30 | 31 | 32 | 33 | 34 => {
                self.op.insert((opcode - 30) as usize, packet.gjstr());
            }
            
            35 | 36 | 37 | 38 | 39 => {
                self.iop.insert((opcode - 35) as usize, packet.gjstr());
            }
            
            40 => {
                let count = packet.g1();
                 
                for i in 0..count {
                    self.recol_s.insert(i as usize, packet.g2());
                    self.recol_d.insert(i as usize, packet.g2());
                }
            }
            
            41 => {
                let count = packet.g1();
                
                for i in 0..count {
                    self.retex_s.insert(i as usize, packet.g2());
                    self.retex_d.insert(i as usize, packet.g2());
                }
            }
            
            42 => {
                let count = packet.g1();
                
                for i in 0..count {
                    self.recol_d_palette.insert(i as usize, packet.g1b());
                }
            }
            
            65 => {
                self.stockmarket_yes = true;
            }
            
            78 => {
                self.manWear3 = packet.g2() as i32;
            }
            
            79 => {
                self.womanWear3 = packet.g2() as i32;
            }
            
            90 => {
                self.manHead = packet.g2() as i32;
            }
            
            91 => {
                self.womanHead = packet.g2() as i32;
            }
            
            92 => {
                self.manHead2 = packet.g2() as i32;
            }
            
            93 => {
                self.womanHead2 = packet.g2() as i32;
            }
            
            95 => {
                self.zand2d = packet.g2() as u32;
            }
            
            96 => {
                self.dummyItem = packet.g1();
            }
            
            97 => {
                self.certlink = packet.g2() as i32;
            }
            
            98 => {
                self.certtemplate = packet.g2() as i32;
            }
            
            100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 => {
                if self.countobj.is_none() {
                    self.countobj = Some(Vec::with_capacity(10));
                    self.countco = Some(Vec::with_capacity(10));
                }
                
                self.countobj.as_mut().unwrap().insert((opcode - 100) as usize, packet.g2());
                self.countco.as_mut().unwrap().insert((opcode - 100) as usize, packet.g2());
            }
            
            110 => {
                self.resizex = packet.g2() as u32;
            }
            
            111 => {
                self.resizey = packet.g2() as u32;
            }
            
            112 => {
                self.resizex = packet.g2() as u32;
            }
            
            113 => {
                self.ambient = packet.g1b();
            }
            
            114 => {
                self.contrast = (packet.g1b() as i32) * 5;
            }
            
            115 => {
                self.team = packet.g1();
            }
            
            121 => {
                self.lentlink = packet.g2() as i32;
            }
            
            122 => {
                self.lenttemplate = packet.g2() as i32;
            }
            
            125 => {
                self.manwearxoffset = packet.g1b();
                self.manwearyoffset = packet.g1b();
                self.manwearzoffset = packet.g1b();
            }
            
            126 => {
                self.womanwearxoffset = packet.g1b();
                self.womanwearyoffset = packet.g1b();
                self.womanwearzoffset = packet.g1b();
            }
            
            127 => {
                self.cursor1op = packet.g1() as i8;
                self.cursor1 = packet.g2() as i32;
            }
            
            128 => {
                self.cursor2op = packet.g1() as i8;
                self.cursor2 = packet.g2() as i32;
            }
            
            249 => {
                self.params = decode_params(packet);
            }

            250 => {
                self.debugname = Some(packet.gjstr());
            }
            _ => { 
                error!("Unknown 'obj' opcode: {}", opcode);
            }
        }
    }
}

pub fn write_obj(file: &mut File, obj: &mut ObjType, opcode_order: &mut Vec<u8>) {
    let mut buffer: Vec<String> = Vec::new();
    
    if obj.debugname.is_some() {
        buffer.push(obj.debugname.clone().unwrap().to_string());   
    } else {
        buffer.push(format!("[obj_{}]", obj.id));    
    }

    // Name always written first if it exists.
    if let Some(name) = &obj.name {
        buffer.push(format!("name={}", name));
    }

    // Examines aren't stored in cache after 377, so position can't be verified
    // Writes out after 'name' if field is populated, writes out blank entry.
    if obj.name.is_some() {
        buffer.push("desc=HIDDEN".to_string());
    }

    let mut model_index: Option<usize> = None;
    let mut recol_index: Option<usize> = None;
    let mut recol_palette_index: Option<usize> = None;
    let mut retex_index: Option<usize> = None;

    for opcode in opcode_order.iter() {
        let dereferenced_opcode = *opcode;

        match dereferenced_opcode {
            0 => {
                break;
            }

            1 => {
                let model_line = format!("model=model_{}_obj", obj.model);
                model_index = Some(buffer.len()); // Save the index for later use.
                buffer.push(model_line);            
            }

            2 => { /* Written separately. */ }

            4 => buffer.push(format!("2dzoom={}", obj.zoom2d)),

            5 => buffer.push(format!("2xanof={}", obj.xan2d)),

            6 => buffer.push(format!("2yanof={}", obj.yan2d)),

            7 => buffer.push(format!("2dxof={}", obj.xof2d)),

            8 => buffer.push(format!("2dyof={}", obj.yof2d)),

            11 => buffer.push("stackable=yes".to_string()),

            12 => buffer.push(format!("cost={}", obj.cost)),

            16 => buffer.push("members=yes".to_string()),

            23 => buffer.push(format!("manwear=model_{}_obj_wear", obj.manWear)),

            24 => buffer.push(format!("manwear2=model_{}_obj_wear", obj.manWear2)),

            25 => buffer.push(format!("womanwear=model_{}_obj_wear", obj.womanWear)),

            26 => buffer.push(format!("womanwear2=model_{}_obj_wear", obj.womanWear2)),

            30..=34 => {
                if obj.op[(opcode - 30) as usize] != "" {
                    buffer.push(format!(
                        "op{}={}",
                        opcode - 29,
                        obj.op[(opcode - 30) as usize]
                    ))
                }
            }

            35..=39 => {
                if obj.iop[(opcode - 35) as usize] != "" {
                    buffer.push(format!(
                        "iop{}={}",
                        opcode - 34,
                        obj.iop[(opcode - 35) as usize]
                    ))
                }
            },

            40 => {
                if let Some(index) = model_index {
                    let mut recol_lines = Vec::new();
                    let recol_count = obj.recol_s.len();

                    // TODO - RGB15 reversal to HSL remains.

                    for i in 0..recol_count {
                        recol_lines.push(format!("recol{}s={}", i + 1, obj.recol_s[i]));
                        recol_lines.push(format!("recol{}d={}", i + 1, obj.recol_d[i]));
                    }

                    let insert_index = index + 1;
                    let recol_len = recol_lines.len();

                    buffer.splice(insert_index..insert_index, recol_lines);
                    recol_index = Some(insert_index + recol_len - 1);
                }
            },

            41 => {
                let index = recol_palette_index.or(recol_index).or(model_index);
                let mut retex_lines = Vec::new();
                let retex_count = obj.retex_s.len();

                for i in 0..retex_count {
                    retex_lines.push(format!("retex{}s={}", i + 1, obj.retex_s[i]));
                    retex_lines.push(format!("retex{}d={}", i + 1, obj.retex_d[i]));
                }

                let insert_index = index.unwrap() + 1;
                let retex_len = retex_lines.len();

                buffer.splice(insert_index..insert_index, retex_lines);
                retex_index = Some(insert_index + retex_len - 1);
            },

            42 => {
                let index = recol_index.or(model_index);
                let mut recol_palette_lines = Vec::new();
                let recol_palette_count = obj.recol_d_palette.len();

                for i in 0..recol_palette_count {
                    recol_palette_lines.push(format!("recol_pal{}d={}", i + 1, obj.recol_d_palette[i]));
                }
                
                let insert_index = index.unwrap() + 1;
                let recol_palette_len = recol_palette_lines.len();

                buffer.splice(insert_index..insert_index, recol_palette_lines);
                recol_index = Some(insert_index + recol_palette_len - 1);
            },

            65 => buffer.push("stockmarket=yes".to_string()),

            78 => buffer.push(format!("manwear3=model_{}_obj_wear", obj.manWear3)),

            79 => buffer.push(format!("womanwear3=model_{}_obj_wear", obj.womanWear3)),
            
            90 => buffer.push(format!("manhead=model_{}_obj_head", obj.manHead)),
            
            91 => buffer.push(format!("womanhead=model_{}_obj_head", obj.womanHead)),
            
            92 => buffer.push(format!("manhead2=model_{}_obj_head", obj.manHead2)),
            
            93 => buffer.push(format!("womanhead2=model_{}_obj_head", obj.womanHead2)),
            
            95 => buffer.push(format!("2dzan={}", obj.zand2d)),
            
            96 => buffer.push(format!("dummyitem=dummy_obj_{}", obj.dummyItem)),

            100..=109 => buffer.push(format!(
                "count{}=obj_{},{}",
                opcode - 99,
                if obj.countobj.is_some() {
                    obj.countobj.as_ref().unwrap()[(opcode - 100) as usize]
                } else {
                    0
                },
                if obj.countco.is_some() {
                    obj.countco.as_ref().unwrap()[(opcode - 100) as usize]
                } else {
                    0
                }
            )),
            
            110 => buffer.push(format!("resizex={}", obj.resizex)),
            
            111 => buffer.push(format!("resizey={}", obj.resizey)),
            
            112 => buffer.push(format!("resizez={}", obj.resizez)),
            
            113 => buffer.push(format!("ambient={}", obj.ambient)),
            
            114 => buffer.push(format!("contrast={}", obj.contrast)),
            
            115 => buffer.push(format!("team={}", obj.team)),
            
            125 => {
                buffer.push(format!(
                    "manwearoffset={},{},{}",
                    obj.manwearxoffset,
                    obj.manwearyoffset,
                    obj.manwearzoffset
                ))
            },
            
            126 => {
                buffer.push(format!(
                    "womanwearoffset={},{},{}",
                    obj.womanwearxoffset,
                    obj.womanwearyoffset,
                    obj.womanwearzoffset
                ))
            }
            
            127 => {
                buffer.push(format!("cursor1op={}", obj.cursor1op));
                buffer.push(format!("cursor1={}", obj.cursor1));
            }
            
            128 => {
                buffer.push(format!("cursor2op={}", obj.cursor2op));
                buffer.push(format!("cursor2={}", obj.cursor2));
            }
            
            249 => {
                for param in obj.params.iter() {
                    let value_str = match param.1 {
                        ParamValue::String(s) => s.clone(),
                        ParamValue::Integer(i) => i.to_string(),
                    };
                    buffer.push(format!("param=param_{},{}", param.0, value_str));
                }
            }
            
            _ => {
                if dereferenced_opcode != 97 && dereferenced_opcode != 98 && dereferenced_opcode != 121 && dereferenced_opcode != 122 {
                    debug!("opcode={}", dereferenced_opcode);    
                }
            }
        }
    }

    if obj.certlink == -1 && obj.stackable == false {
        buffer.push("tradeable=no".parse().unwrap());
    }
    
    // Verify buffer length, if single-line, it's an empty cert obj.
    if !buffer.is_empty() && buffer.len() > 1 {
        buffer.push("".to_string());

        for line in buffer {
            writeln!(file, "{}", line).unwrap();
        }
    }
}