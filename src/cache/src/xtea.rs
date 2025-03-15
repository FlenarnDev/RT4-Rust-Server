#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XTEAKey {
    pub k0: i32,
    pub k1: i32,
    pub k2: i32,
    pub k3: i32,
}

impl XTEAKey {
    pub fn new(k0: i32, k1: i32, k2: i32, k3: i32) -> Self {
        XTEAKey { k0, k1, k2, k3 }
    }
    
    pub const ZERO: Self = XTEAKey { k0: 0, k1: 0, k2: 0, k3: 0 };
    
    pub fn is_zero(&self) -> bool {
        self.k0 == 0 && self.k1 == 0 && self.k2 == 0 && self.k3 == 0
    }
    
    /// Convert to array
    pub fn to_array(&self) -> [i32; 4] {
        [self.k0, self.k1, self.k2, self.k3]
    }
}