use lazy_static::lazy_static;

lazy_static! {
    pub static ref MASK: [i32; 33] = init_mask_array();
}

/// Returns the number of `1` bits in `value`.
pub fn bitcount(value: i32) -> i32 {
    let mut n = value;
    n = n - ((n >> 1) & 0x55555555);
    n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
    ((n + (n >> 4) & 0x0f0f0f0f).wrapping_mul(0x01010101)) >> 24
}

/// Sets a range of bits from `start_bit` to `end_bit` (inclusive) to 1.
pub fn set_bit_range(value: i32, start_bit: i32, end_bit: i32) -> i32 {
    let mask = MASK[(end_bit - start_bit + 1) as usize];
    value | (mask << start_bit)
}

/// Clears a range of bits from `start_bit` to `end_bit` to 0.
pub fn clear_bit_range(value: i32, start_bit: i32, end_bit: i32) -> i32 {
    let mask = MASK[(end_bit - start_bit + 1) as usize];
    value & !(mask << start_bit)
}

/// Initialize the array of bit masks.
fn init_mask_array() -> [i32; 33] {
    let mut data = [0; 33];
    let mut incrementor = 2;
    
    for i in 1..33 {
        data[i] = (incrementor - 1);
        incrementor += incrementor;
    }
    
    data
}