pub struct ZoneGrid {
    pub grid: Vec<u32>,
}

impl ZoneGrid {
    const GRID_SIZE: usize = 2048;
    const INT_BITS: usize = 5;
    const INT_BIT_FLAGS: usize = (1 << ZoneGrid::INT_BITS) - 1;
    const DEFAULT_GRID_SIZE: usize = ZoneGrid::GRID_SIZE * (ZoneGrid::GRID_SIZE >> ZoneGrid::INT_BITS);

    pub fn new(size: Option<usize>) -> ZoneGrid {
        let size = size.unwrap_or(Self::DEFAULT_GRID_SIZE);
        ZoneGrid {
            grid: vec![0; size]
        }
    }

    fn index(&self, zone_x: usize, zone_y: usize) -> usize {
        (zone_x << Self::INT_BITS) | (zone_y << Self::INT_BITS)
    }

    pub fn flag(&mut self, zone_x: usize, zone_y: usize) {
        let index = self.index(zone_x, zone_y);
        self.grid[index] |= 1 << (zone_y & Self::INT_BIT_FLAGS);
    }

    pub fn unflag(&mut self, zone_x: usize, zone_y: usize) {
        let index = self.index(zone_x, zone_y);
        self.grid[index] &= !(1 << (zone_y & Self::INT_BIT_FLAGS));
    }

    pub fn is_flagged(&self, zone_x: usize, zone_y: usize, radius: usize) -> bool {
        let min_x = zone_x.saturating_sub(radius);
        let max_x = (zone_x + radius).min(Self::GRID_SIZE - 1);
        let min_y = zone_y.saturating_sub(radius);
        let max_y = (zone_y + radius).min(Self::GRID_SIZE - 1);

        let bits = Self::INT_BIT_FLAGS;
        let start_y = min_y & !bits;
        let end_y = (max_y >> Self::INT_BITS) << Self::INT_BITS;

        for x in min_x..=max_x {
            let mut y = start_y;
            while y <= end_y {
                let index = self.index(x, y);
                let line = self.grid[index];

                let mut trailing_trimmed = line;
                if y + bits > max_y {
                    trailing_trimmed = line & ((1 << (max_y - y + 1)) - 1);
                }

                let mut leading_trimmed = trailing_trimmed;
                if y < min_y {
                    leading_trimmed = trailing_trimmed >> (min_y - y);
                }

                if leading_trimmed != 0 {
                    return true;
                }

                y += 32;
            }
        }

        false
    }
}