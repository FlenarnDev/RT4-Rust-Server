#[derive(Clone)]
pub struct Isaac {
    rsl: Vec<i32>,
    mem: Vec<i32>,
    count: usize,
    a: i32,
    b: i32,
    c: i32,
}

impl Isaac {
    pub fn new(seed: Vec<i32>) -> Isaac {
        let mut isaac: Isaac = Isaac {
            rsl: vec![0; 256],
            mem: vec![0; 256],
            count: 0,
            a: 0,
            b: 0,
            c: 0,
        };
        isaac.rsl[..seed.len()].copy_from_slice(&seed);
        isaac.init();
        isaac
    }
    
    pub fn next(&mut self) -> i32 {
        if self.count == 0 {
            self.isaac();
            self.count = 255;
        } else {
            self.count -= 1;
        }
        self.rsl[self.count]
    }
    
    fn init(&mut self) {
        let mut a: i32 = 0x9e3779b9u32 as i32;
        let mut b: i32 = 0x9e3779b9u32 as i32;
        let mut c: i32 = 0x9e3779b9u32 as i32;
        let mut d: i32 = 0x9e3779b9u32 as i32;
        let mut e: i32 = 0x9e3779b9u32 as i32;
        let mut f: i32 = 0x9e3779b9u32 as i32;
        let mut g: i32 = 0x9e3779b9u32 as i32;
        let mut h: i32 = 0x9e3779b9u32 as i32;

        // mix
        for _ in 0..4 {
            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);
        }

        // first pass
        for index in (0..256).step_by(8) {
            // rsl
            a = a.wrapping_add(self.rsl[index]);
            b = b.wrapping_add(self.rsl[index + 1]);
            c = c.wrapping_add(self.rsl[index + 2]);
            d = d.wrapping_add(self.rsl[index + 3]);
            e = e.wrapping_add(self.rsl[index + 4]);
            f = f.wrapping_add(self.rsl[index + 5]);
            g = g.wrapping_add(self.rsl[index + 6]);
            h = h.wrapping_add(self.rsl[index + 7]);

            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);

            // mem
            self.mem[index] = a;
            self.mem[index + 1] = b;
            self.mem[index + 2] = c;
            self.mem[index + 3] = d;
            self.mem[index + 4] = e;
            self.mem[index + 5] = f;
            self.mem[index + 6] = g;
            self.mem[index + 7] = h;
        }

        // second pass
        for index in (0..256).step_by(8) {
            a = a.wrapping_add(self.mem[index]);
            b = b.wrapping_add(self.mem[index + 1]);
            c = c.wrapping_add(self.mem[index + 2]);
            d = d.wrapping_add(self.mem[index + 3]);
            e = e.wrapping_add(self.mem[index + 4]);
            f = f.wrapping_add(self.mem[index + 5]);
            g = g.wrapping_add(self.mem[index + 6]);
            h = h.wrapping_add(self.mem[index + 7]);

            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);

            self.mem[index] = a;
            self.mem[index + 1] = b;
            self.mem[index + 2] = c;
            self.mem[index + 3] = d;
            self.mem[index + 4] = e;
            self.mem[index + 5] = f;
            self.mem[index + 6] = g;
            self.mem[index + 7] = h;
        }

        self.isaac();
        self.count = 256;
    }
    
    fn isaac(&mut self) {
        self.c = self.c.wrapping_add(1);
        self.b = self.b.wrapping_add(self.c);
        
        for index in 0..256 {
            let x: i32 = self.mem[index];
            
            match index & 3 {
                0 => self.a ^= self.a << 13,
                1 => self.a ^= (self.a as u32 >> 6) as i32,
                2 => self.a ^= self.a << 2,
                3 => self.a ^= (self.a as u32 >> 16) as i32,
                _ => {}
            }
            
            self.a = self.a.wrapping_add(self.mem[(index + 128) & 0xFF]);
            
            let y: i32 = self.mem[((x >> 2) & 0xFF) as usize]
                .wrapping_add(self.a)
                .wrapping_add(self.b);
            self.mem[index] = y;
            
            self.b = self.mem[((y >> 10) & 0xFF) as usize].wrapping_add(x);
            self.rsl[index] = self.b;
        }
    }
}