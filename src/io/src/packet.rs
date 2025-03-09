#[derive(Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub position: usize,
    pub bit_position: usize,
}

impl Packet {
    /// Create a 'Packet' with a fixed sized allocated buffer.
    pub fn new(size: usize) -> Packet {
        Packet {
            data: Vec::with_capacity(size),
            position: 0,
            bit_position: 0,
        }
    }

    /// Create a new 'Packet' from a 'Vec<u8>' array.
    /// This will take ownership of the input vector.
    pub fn from(data: Vec<u8>) -> Packet {
        Packet {
            data,
            position: 0,
            bit_position: 0,
        }
    }

    /// Create a new 'Packet' from an input file from IO.
    pub fn io(path: String) -> Packet {
        Packet::from(std::fs::read(path).unwrap())
    }

    /// Returns the remaining amount of storage available for this 'Packet'.
    /// This is calculated by the difference of the total length with the current
    /// position of this packet.
    #[inline(always)]
    pub fn remaining(&self) -> i32 {
        (self.len() - self.position) as i32
    }

    /// Returns the total usize (length) of this 'Packet' allocated for storage of bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn p1(&mut self, value: i32) {
        self.data.push(value as u8);
        self.position += 1;
    }

    #[inline(always)]
    pub fn p2(&mut self, value: i32) {
        let truncated_value = value as u16;
        self.data.extend_from_slice(&truncated_value.to_be_bytes());
        self.position += 2;
    }

    #[inline(always)]
    pub fn ip2(&mut self, value: i32) {
        let start: usize = self.position;
        unsafe { self.data.get_unchecked_mut(start..start + 2) }
            .copy_from_slice(&(value as u16).to_le_bytes());
        self.position += 2;
    }

    #[inline(always)]
    pub fn p3(&mut self, value: i32) {
        let start: usize = self.position;
        unsafe { *self.data.get_unchecked_mut(start) = (value >> 16) as u8 };
        unsafe { self.data.get_unchecked_mut(start + 1..start + 3) }
            .copy_from_slice(&(value as u16).to_be_bytes());
        self.position += 3;
    }

    #[inline(always)]
    pub fn p4(&mut self, value: i32) {
        self.data.extend_from_slice(&value.to_be_bytes());
        self.position += 4;
    }

    #[inline(always)]
    pub fn ip4(&mut self, value: i32) {
        let start: usize = self.position;
        unsafe { self.data.get_unchecked_mut(start..start + 4) }
            .copy_from_slice(&value.to_le_bytes());
        self.position += 4;
    }

    #[inline(always)]
    pub fn p8(&mut self, value: i64) {
        self.data.extend_from_slice(&value.to_be_bytes());
        self.position += 8;
    }

    #[inline(always)]
    pub fn pjstr(&mut self, str: &str, terminator: u8) {
        let required_len = self.position + str.len() + 1;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let mut length = self.position;
        for byte in str.bytes() {
            self.data[length] = byte;
            length += 1;
        }

        self.data[length] = terminator;
        self.position = length + 1;
    }

    #[inline(always)]
    pub fn pjstr2(&mut self, str: &str) {
        self.p1(0);
        self.pjstr(str, 0);
    }

    #[inline(always)]
    pub fn psmart(&mut self, value: i32) {
        if value >= 0 && value < 128 {
            self.p1(value);
        } else if value >= 0 && value < 32768 {
            self.p2(value + 32768);
        } else {
            panic!("Error psmart out of range: {}", value);
        }
    }

    #[inline(always)]
    pub fn psmarts(&mut self, value: i32) {
        if value < 64 && value >= -64 {
            self.p1(value + 64);
        } else if value < 16384 && value >= -16384 {
            self.p2(value + 49152);
        } else {
            panic!("Error psmarts out of range: {}", value);
        }
    }

    #[inline(always)]
    pub fn pbytes(&mut self, src: &Vec<u8>, offset: usize, length: usize) {
        for i in 0..length
        {
            self.data.push(src[offset + i]);
            self.position += 1;
        }
    }

    #[inline(always)]
    pub fn g1(&mut self) -> u8 {
        self.position += 1;
        unsafe { *self.data.get_unchecked(self.position - 1) }
    }

    #[inline(always)]
    pub fn g1s(&mut self) -> i8 {
        self.position += 1;
        (unsafe { *self.data.get_unchecked(self.position - 1) }) as i8
    }

    #[inline(always)]
    pub fn g2(&mut self) -> u16 {
        self.position += 2;
        let pos: usize = self.position;
        u16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        )
    }

    #[inline(always)]
    pub fn g2s(&mut self) -> i16 {
        self.position += 2;
        let pos: usize = self.position;
        i16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        )
    }

    #[inline(always)]
    pub fn ig2s(&mut self) -> i16 {
        self.position += 2;
        let pos: usize = self.position;
        i16::from_le_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        )
    }

    #[inline(always)]
    pub fn g3(&mut self) -> i32 {
        self.position += 3;
        let pos: usize = self.position;
        ((unsafe { *self.data.get_unchecked(pos - 3) } as u32) << 16
            | u16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        ) as u32) as i32
    }

    #[inline(always)]
    pub fn g4(&mut self) -> i32 {
        self.position += 4;
        let pos: usize = self.position;

        if pos < 4 {
            return 0; // Prevent out-of-bounds access
        }

        let b1 = self.data.get(pos - 4).copied().unwrap_or(0) as u32;
        let b2 = self.data.get(pos - 3).copied().unwrap_or(0) as u32;
        let b3 = self.data.get(pos - 2).copied().unwrap_or(0) as u32;
        let b4 = self.data.get(pos - 1).copied().unwrap_or(0) as u32;

        ((b1 << 24) | (b2 << 16) | (b3 << 8) | b4) as i32
    }

    #[inline(always)]
    pub fn g4s(&mut self) -> i32 {
        self.position += 4;
        let pos: usize = self.position;
        i32::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 4..pos) }
                .try_into()
                .unwrap(),
        )
    }

    #[inline(always)]
    pub fn ig4s(&mut self) -> i32 {
        self.position += 4;
        let pos: usize = self.position;
        return i32::from_le_bytes(
            unsafe { self.data.get_unchecked(pos - 4..pos) }
                .try_into()
                .unwrap(),
        );
    }

    #[inline(always)]
    pub fn g8s(&mut self) -> i64 {
        self.position += 8;
        let pos: usize = self.position;
        i64::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 8..pos) }
                .try_into()
                .unwrap(),
        )
    }

    /// Reads a string from the internal buffer until a terminator byte is encountered.
    #[inline(always)]
    pub fn gjstr(&mut self, terminator: u8) -> String {
        let pos: usize = self.position;
        let mut length = pos;
        while unsafe { *self.data.get_unchecked(length) } != terminator {
            length += 1;
        }
        let str: &str =
            unsafe { std::str::from_utf8_unchecked(&self.data.get_unchecked(pos..length)) };
        self.position = length + 1;
        str.to_owned()
    }

    #[inline(always)]
    pub fn gsmart(&mut self) -> i32 {
        if unsafe { *self.data.get_unchecked(self.position) } < 128 {
            self.g1() as i32
        } else {
            self.g2() as i32 - 32768
        }
    }

    #[inline(always)]
    pub fn gsmarts(&mut self) -> i32 {
        if unsafe { *self.data.get_unchecked(self.position) } < 128 {
            self.g1() as i32 - 64
        } else {
            self.g2() as i32 - 49152
        }
    }

    #[inline(always)]
    pub fn gbytes(&mut self, length: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(length);
        for i in 0..length {
            result.push(self.data[self.position + i]);
        }
        self.position += length;
        result
    }

    /// Sets the internal bit position (`bit_pos`) to the current byte position (`pos`)
    /// converted to bit position. This is typically used when switching from byte-based
    /// to bit-based addressing.
    #[inline(always)]
    pub fn bits(&mut self) {
        self.bit_position = self.position << 3;
    }

    /// Sets the internal byte position (`pos`) based on the current bit position (`bit_pos`).
    /// This is typically used when switching from bit-based addressing back to byte-based addressing.
    #[inline(always)]
    pub fn bytes(&mut self) {
        self.position = (self.bit_position + 7) >> 3;
    }
}