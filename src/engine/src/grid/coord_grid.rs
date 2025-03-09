#[derive(Eq, Hash, PartialEq)]
pub struct CoordGrid {
    pub coord: u32,
}

impl CoordGrid {
    #[inline(always)]
    pub fn new(coord: u32) -> CoordGrid {
        CoordGrid { coord }
    }
    
    #[inline(always)]
    pub fn from(x: u16, y: u8, z: u16) -> CoordGrid {
        return CoordGrid {
            coord: ((z & 0x3FFF) as u32)
            | (((x & 0x3FFF) as u32) << 14)
            | (((y & 0x3F) as u32) << 28)
        };
    }
    
    #[inline(always)]
    pub fn zone_coord(&self) -> u8 {
        return (((self.x() & 0x7) as u8) << 4) | ((self.z() & 0x7) as u8);
    }
    
    #[inline(always)]
    pub fn y(&self) -> u8 {
        return (self.coord >> 28 & 0x3) as u8;
    }

    #[inline(always)]
    pub fn x(&self) -> u16 {
        return (self.coord >> 14 & 0x3FFF) as u16;
    }
    
    #[inline(always)]
    pub fn z(&self) -> u16 {
        return (self.coord & 0x3FFF) as u16;
    }

    #[inline(always)]
    pub fn distance(&self, x: u16, y: u8, z: u16) -> CoordGrid {
        return CoordGrid::from(
            self.x().wrapping_add(x),
            self.y().wrapping_add(y),
            self.z().wrapping_add(z),
        );
    }
    
    #[inline(always)]
    pub fn movecoord_other(&self, other: CoordGrid) -> CoordGrid {
        return CoordGrid::from(
            self.x().wrapping_add(other.x()),
            self.y().wrapping_add(other.y()),
            self.z().wrapping_add(other.z()),
        )
    }
}