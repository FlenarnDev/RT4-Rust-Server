use std::collections::HashMap;
use smallvec::smallvec;
use crate::io::packet::Packet;
use crate::util::cache::config::config_type::ConfigType;
use crate::util::cache::param_helper::{decode_params, ParamValue, Params};

pub struct ObjType {
    id: u32,
    debugname: Option<String>,
    model: u32,
    name: Option<String>,
    recol_s: Vec<u16>,
    recol_d: Vec<u16>,
    retex_s: Vec<u16>,
    retex_d: Vec<u16>,
    recol_d_palette: Vec<i8>,
    zoom2d: u32,
    xan2d: u32,
    yan2d: u32,
    xof2d: u32,
    yof2d: u32,
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
            op: Vec::with_capacity(5),
            iop: Vec::with_capacity(5),
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
                self.name = Some(packet.gjstr(0));
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
                self.xof2d = packet.g2() as u32;
            }
            
            8 => {
                self.yof2d = packet.g2() as u32;
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
                self.op.insert((opcode - 30) as usize, packet.gjstr(0))
            }
            
            35 | 36 | 37 | 38 | 39 => {
                self.iop.insert((opcode - 35) as usize, packet.gjstr(0))
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
                self.contrast = (packet.g1b() * 5) as i32;
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
            _ => todo!(),
        }
    }
}