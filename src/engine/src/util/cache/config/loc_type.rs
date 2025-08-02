use std::fs::File;
use log::{debug, error};
use crate::entity::loc::Loc;
use crate::io::packet::Packet;
use crate::util::cache::config::config_type::ConfigType;
use crate::util::cache::param_helper::{decode_params, ParamValue, Params};
use std::io::Write;

#[derive(Debug)]
pub struct LocType {
    pub id: u32,
    debugname: Option<String>,
    pub models: Vec<u32>,
    pub ldmodels: Vec<u32>,
    pub shapes: Vec<u32>,
    pub ldshapes: Vec<u32>,
    pub name: Option<String>,
    description: Option<String>,
    width: u32,
    length: u32,
    recol_s: Vec<u16>,
    recol_d: Vec<u16>,
    retex_s: Vec<u16>,
    retex_d: Vec<u16>,
    recol_d_palette: Vec<i8>,
    blockwalk: u32,
    blockrange: bool,
    blocksides: u32,
    members: bool,
    op: Vec<String>,
    active: i8,
    sharelight: bool,
    occlude: bool,
    anim: i32,
    ambient: i8,
    contrast: i16,
    resizex: u32,
    resizey: u32,
    resizez: u32,
    walloff: u32,
    hillchange: u32,
    hillchange_amount: i32,
    mapfunction: i16,
    multivarbit: i32,
    multivarp: i32,
    multiloc: Vec<i32>,
    istexture: bool,
    shadow: bool,
    randomanimframe: bool,
    renderunderfeet: bool, // TODO: Name is guessed (and badly so).
    xoff: u32,
    yoff: u32,
    zoff: u32,
    bgsound_sound: i32,
    bgsound_range: u8,
    bgsound_mindelay: u16,
    bgsound_maxdelay: u16,
    bgsound_random: Vec<u16>,
    hasanimation: bool, // TODO: Name is guessed.
    mirror: bool,
    hardshadow: bool,
    mapsceneiconrotate: bool,
    forcedecor: bool,
    breakroutefinding: bool,
    animated: bool,
    raiseobject: i32,
    mapsceneiconrotationoffset: u8,
    mapsceneicon: i16,
    cursor1op: i8,
    cursor1: i32,
    cursor2op: i8,
    cursor2: i32,
    params: Params,
}

impl LocType {
    pub fn new(id: u32) -> Self {
        LocType {
            id,
            debugname: None,
            models: Vec::new(),
            ldmodels: Vec::new(),
            shapes: Vec::new(),
            ldshapes: Vec::new(),
            name: None,
            description: None,
            width: 1,
            length: 1,
            recol_s: Vec::new(),
            recol_d: Vec::new(),
            retex_s: Vec::new(),
            retex_d: Vec::new(),
            recol_d_palette: Vec::new(),
            blockwalk: 2,
            blockrange: true,
            blocksides: 0,
            members: false,
            op: vec!["".to_string(); 5],
            active: -1,
            sharelight: false,
            occlude: false,
            anim: -1,
            ambient: 0,
            contrast: 0,
            resizex: 128,
            resizey: 128,
            resizez: 128,
            walloff: 16,
            hillchange: 0,
            hillchange_amount: -1,
            mapfunction: -1,
            multivarbit: -1,
            multivarp: -1,
            multiloc: Vec::new(),
            istexture: false,
            shadow: true,
            randomanimframe: true,
            renderunderfeet: false,
            xoff: 0,
            yoff: 0,
            zoff: 0,
            bgsound_sound: -1,
            bgsound_range: 0,
            bgsound_mindelay: 0,
            bgsound_maxdelay: 0,
            bgsound_random: Vec::new(),
            hasanimation: false,
            mirror: false,
            hardshadow: true,
            mapsceneiconrotate: false,
            forcedecor: false,
            breakroutefinding: false,
            animated: false,
            raiseobject: -1,
            mapsceneiconrotationoffset: 0,
            mapsceneicon: -1,
            cursor1op: -1,
            cursor1: -1,
            cursor2op: -1,
            cursor2: -1,
            params: Params::default(),
        }
    }
}

impl ConfigType for LocType {
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
            1 | 5 => {
                let count = packet.g1();
                
                if self.models.is_empty() {
                    self.models = Vec::with_capacity(count as usize);
                    self.shapes = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        self.models.push(packet.g2() as u32);
                        if opcode == 1 {
                            self.shapes.push(packet.g1() as u32);
                        }
                        else {
                            self.shapes.push(10);
                        }
                    }
                } else {
                    self.ldmodels = Vec::with_capacity(count as usize);
                    self.ldshapes = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        self.ldmodels.push(packet.g2() as u32);
                        if opcode == 1 {
                            self.ldshapes.push(packet.g1() as u32);
                        }
                        else {
                            self.ldshapes.push(10);
                        }
                    }
                }
            }

            2 => {
                self.name = Some(packet.gjstr());
            }

            14 => {
                self.width = packet.g1() as u32;
            }

            15 => {
                self.length = packet.g1() as u32;
            }

            17 => {
                self.blockwalk = 0;
                self.blockrange = false;
            }

            18 => {
                self.blockrange = false;
            }

            19 => {
                self.active = packet.g1() as i8;
            }

            21 => {
                self.hillchange = 1;
            }

            22 => {
                self.sharelight = true;
            }

            23 => {
                self.occlude = true;
            }

            24 => {
                self.anim = packet.g2() as i32;
                if self.anim == 65535 {
                    self.anim = -1;
                }
            }

            27 => {
                self.blockwalk = 1;
            }

            28 => {
                self.walloff = packet.g1() as u32;
            }

            29 => {
                self.ambient = packet.g1b();
            }

            30 | 31 | 32 | 33 |34 => {
                self.op.insert((opcode - 30) as usize, packet.gjstr())
            }

            39 => {
                self.contrast = packet.g1b() as i16 * 5;
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

            60 => {
                self.mapfunction = packet.g2() as i16;
            }

            62 => {
                self.mirror = true;
            }

            64 => {
                self.shadow = false;
            }

            65 => {
                self.resizex = packet.g2() as u32;
            }

            66 => {
                self.resizey = packet.g2() as u32;
            }

            67 => {
                self.resizez  = packet.g2() as u32;
            }

            69 => {
                self.blocksides = packet.g1() as u32;
            }

            70 => {
                self.xoff = packet.g2s() as u32
            }

            71 => {
                self.yoff = packet.g2s() as u32
            }

            72 => {
                self.zoff = packet.g2s() as u32
            }

            73 => {
                self.forcedecor = true;
            }

            74 => {
                self.breakroutefinding = true;
            }

            75 => {
                self.raiseobject =  packet.g1() as i32;
            }

            77 | 92 => {
                self.multivarbit = packet.g2() as i32;
                if self.multivarbit == 65535 {
                    self.multivarbit = -1;
                }

                self.multivarp = packet.g2() as i32;
                if self.multivarp == 65535 {
                    self.multivarp = -1;
                }

                let mut default_id = -1;
                if opcode == 92 {
                    default_id = packet.g2() as i32;
                    if default_id == 65535 {
                        default_id = -1;
                    }
                }

                let length = packet.g1();
                self.multiloc = Vec::with_capacity((length as usize + 2));
                for _ in 0..=length {
                    let mut value = packet.g2() as i32;
                    if value == 65535 {
                        value = -1;
                    }
                    self.multiloc.push(value);
                }
                self.multiloc.push(default_id);
            }

            78 => {
                self.bgsound_sound = packet.g2() as i32;
                self.bgsound_range = packet.g1();
            }

            79 => {
                self.bgsound_mindelay = packet.g2();
                self.bgsound_maxdelay = packet.g2();
                self.bgsound_range = packet.g1();
                let length = packet.g1();
                self.bgsound_random = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    self.bgsound_random.push(packet.g2());
                }
            }

            81 => {
                self.hillchange = 2;
                self.hillchange_amount = packet.g1() as i32 * 256;
            }

            82 => {
                self.istexture = true;
            }

            88 => {
                self.hardshadow = false;
            }

            89 => {
                self.randomanimframe = false;
            }

            90 => {
                self.renderunderfeet = true;
            }

            91 => {
                self.members = true;
            }

            93 => {
                self.hillchange = 3;
                self.hillchange_amount = packet.g2() as i32;
            }

            94 => {
                self.hillchange = 4;
            }

            95 => {
                self.hillchange = 5;
            }

            96 => {
                self.hasanimation = true;
            }

            97 => {
                self.mapsceneiconrotate = true;
            }
            
            98 => {
                self.animated = true;
            }

            99 => {
                self.cursor1op = packet.g1() as i8;
                self.cursor1 = packet.g2() as i32;
            }

            100 => {
                self.cursor2op = packet.g1() as i8;
                self.cursor2 = packet.g2() as i32;
            }

            101 => {
                self.mapsceneiconrotationoffset = packet.g1();
            }
            
            102 => {
                self.mapsceneicon = packet.g2() as i16;
            }

            249 => {
                self.params = decode_params(packet);
            }
            
            250 => {
                self.debugname = Some(packet.gjstr());
            }

            _ => {
                error!("Unknown 'loc' opcode {}", opcode);
            }
        }
    }
}

pub fn write_loc(file: &mut File, loc: &mut LocType, opcode_order: &mut Vec<u8>) {
    let mut buffer: Vec<String> = Vec::new();

    if loc.debugname.is_some() {
        buffer.push(loc.debugname.clone().unwrap().to_string());
    } else {
        buffer.push(format!("[loc_{}]", loc.id));
    }

    // Name always written first if it's populated.
    if let Some(name) = &loc.name {
        buffer.push(format!("name={}", name));
    }

    // Examines aren't stored in cache after 377, so position can't be verified
    // Writes out after 'name' if the field is populated, writes out blank entry.
    if loc.name.is_some() {
        buffer.push("desc=HIDDEN".to_string());
    }

    let mut model_index: Option<usize> = None;
    let mut recol_index: Option<usize> = None;
    let mut recol_palette_index: Option<usize> = None;
    let mut retex_index: Option<usize> = None;

    // Opcode 5 can be written multiple times - with different intent (first write normal model, second low detail model).
    // Probably not 
    let mut modelsWritten: bool = false;
    
    for opcode in opcode_order.iter() {
        let dereferenced_opcode = *opcode;

        match dereferenced_opcode {
            0 => {
                break;
            }
            
            1 => {
                for model in 0..loc.models.len()
                {
                    buffer.push(format!("model{}=model_{}_loc", model + 1, loc.models[model]));
                }
            }
            
            2 => { /* Written separately. */ }

            5 => {
                if modelsWritten {
                    for ldmodel in 0..loc.ldmodels.len()
                    {
                        buffer.push(format!("ldmodel{}=model_{}_loc", ldmodel + 1, loc.ldmodels[ldmodel]));
                    }
                } else {
                    for model in 0..loc.models.len()
                    {
                        buffer.push(format!("model{}=model_{}_loc", model + 1, loc.models[model]));
                    }
                    modelsWritten = true;
                }
            }
            
            14 => {
                buffer.push(format!("width={}", loc.width));
            }
            
            15 => {
                buffer.push(format!("length={}", loc.length));
            }
            
            29 => buffer.push(format!("ambient={}", loc.ambient)),
            
            30 => buffer.push(format!("contrast={}", loc.contrast / 5)),
            
            30..=34 => {
                if loc.op[(opcode - 30) as usize] != "" {
                    buffer.push(format!(
                        "op{}={}",
                        opcode - 29,
                        loc.op[(opcode - 30) as usize]
                    ))
                }
            }
            
            40 => {
                if let Some(index) = model_index {
                    let mut recol_lines = Vec::new();
                    let recol_count = loc.recol_s.len();
                    
                    // TODO RGB15 reversal to HSL remains.
                    
                    for i in 0..recol_count {
                        recol_lines.push(format!("recol{}s={}", i + 1, loc.recol_s[i]));
                        recol_lines.push(format!("recol{}d={}", i + 1, loc.recol_d[i]));
                    }
                    
                    let insert_index = index + 1;
                    let recol_len = recol_lines.len();
                    
                    buffer.splice(insert_index..index, recol_lines);
                    recol_index = Some(insert_index + recol_len - 1);
                }
            }
            
            41 => {
                let index = recol_palette_index.or(recol_index).or(model_index);
                let mut retex_lines = Vec::new();
                let retex_count = loc.retex_s.len();
                
                for i in 0..retex_count {
                    retex_lines.push(format!("retex{}s={}", i + 1, loc.retex_s[i]));
                    retex_lines.push(format!("retex{}d={}", i + 1, loc.retex_d[i]));
                }
                
                let insert_index = index.unwrap() + 1;
                let retex_len = retex_lines.len();
                
                buffer.splice(insert_index..insert_index, retex_lines);
                retex_index = Some(insert_index + retex_len - 1);
            }
            
            42 => {
                let index = recol_index.or(model_index);
                let mut recol_palette_lines = Vec::new();
                let recol_palette_count = loc.recol_d_palette.len();
                
                for i in 0..recol_palette_count {
                    recol_palette_lines.push(format!("recol_pal{}d={}", i + 1, loc.recol_d_palette[i]));
                }
                
                let insert_index = index.unwrap() + 1;
                let recol_palette_len = recol_palette_lines.len();
                
                buffer.splice(insert_index..insert_index, recol_palette_lines);
                recol_index = Some(insert_index + recol_palette_len - 1);
            }
            
            64 => buffer.push("shadow=no".to_string()),
            
            88 => buffer.push("hardshadow=no".to_string()),

            91 => buffer.push("members=yes".to_string()),

            99 => {
                buffer.push(format!("cursor1op={}", loc.cursor1op));
                buffer.push(format!("cursor1={}", loc.cursor1));
            }

            100 => {
                buffer.push(format!("cursor2op={}", loc.cursor2op));
                buffer.push(format!("cursor2={}", loc.cursor2));
            }

            249 => {
                for param in loc.params.iter() {
                    let value_str = match param.1 {
                        ParamValue::String(s) => s.clone(),
                        ParamValue::Integer(i) => i.to_string(),
                    };
                    buffer.push(format!("param=param_{},{}", param.0, value_str));
                }
            }

            _ => {

            }
        }
    }

    // Verify buffer length, if single-line, it's an empty cert obj.
    if !buffer.is_empty() && buffer.len() > 1 {
        buffer.push("".to_string());

        for line in buffer {
            writeln!(file, "{}", line).unwrap();
        }
    }
}