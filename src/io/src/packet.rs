#[derive(Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub position: usize,
    pub bit_position: usize,
}

// Helper function for branch prediction optimization
macro_rules! likely {
    ($expr:expr) => {
        $expr
    }
}

impl Packet {
    /// Create a 'Packet' with a fixed sized allocated buffer.
    pub fn new(size: usize) -> Packet {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Packet {
            data,
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
    pub fn io(path: String) -> Result<Packet, std::io::Error> {
        Ok(Packet::from(std::fs::read(path)?))
    }

    /// Returns the remaining amount of storage available for this 'Packet'.
    /// This is calculated by the difference of the total length with the current
    /// position of this packet.
    #[inline(always)]
    pub fn remaining(&self) -> i32 {
        (self.len() - self.position) as i32
    }

    /// Returns the total usize (length) of this 'Packet' allocated for storage of bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the packet contains no data.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline(always)]
    pub fn p1(&mut self, value: i32) {
        let value_u8 = value as u8;  // Do the conversion once

        if self.position < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position] = value_u8;
        } else {
            // Slow path - extend buffer
            self.data.push(value_u8);
        }

        self.position += 1;
    }

    #[inline(always)]
    pub fn p2(&mut self, value: i32) {
        let bytes = (value as u16).to_be_bytes();
        if self.position + 1 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position] = bytes[0];
            self.data[self.position + 1] = bytes[1];
        } else {
            // Slow path - extend buffer
            self.data.extend_from_slice(&bytes);
        }
        self.position += 2;
    }

    #[inline(always)]
    pub fn ip2(&mut self, value: i32) {
        let bytes = (value as u16).to_le_bytes();
        if self.position + 1 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position] = bytes[0];
            self.data[self.position + 1] = bytes[1];
        } else {
            // Slow path - extend buffer
            self.data.extend_from_slice(&bytes);
        }
        self.position += 2;
    }

    #[inline(always)]
    pub fn p3(&mut self, value: i32) {
        let b1 = (value >> 16) as u8;
        let bytes = (value as u16).to_be_bytes();

        if self.position + 2 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position] = b1;
            self.data[self.position + 1] = bytes[0];
            self.data[self.position + 2] = bytes[1];
        } else {
            // Slow path - extend buffer
            self.data.push(b1);
            self.data.extend_from_slice(&bytes);
        }
        self.position += 3;
    }

    #[inline(always)]
    pub fn p4(&mut self, value: i32) {
        let bytes = value.to_be_bytes();
        if self.position + 3 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position..self.position + 4].copy_from_slice(&bytes);
        } else {
            // Slow path - extend buffer
            self.data.extend_from_slice(&bytes);
        }
        self.position += 4;
    }

    #[inline(always)]
    pub fn ip4(&mut self, value: i32) {
        let bytes = value.to_le_bytes();
        if self.position + 3 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position..self.position + 4].copy_from_slice(&bytes);
        } else {
            // Slow path - extend buffer
            self.data.extend_from_slice(&bytes);
        }
        self.position += 4;
    }

    #[inline(always)]
    pub fn p8(&mut self, value: i64) {
        let bytes = value.to_be_bytes();
        if self.position + 7 < self.data.len() {
            // Fast path - write to existing buffer
            self.data[self.position..self.position + 8].copy_from_slice(&bytes);
        } else {
            // Slow path - extend buffer
            self.data.extend_from_slice(&bytes);
        }
        self.position += 8;
    }

    #[inline(always)]
    pub fn pjstr(&mut self, str: &str, terminator: u8) {
        let bytes = str.as_bytes();
        let required_len = self.position + bytes.len() + 1;

        // Ensure we have enough space
        if required_len > self.data.len() {
            self.data.resize(required_len, 0);
        }

        // Copy the string bytes
        self.data[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.data[self.position + bytes.len()] = terminator;
        self.position = self.position + bytes.len() + 1;
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
    pub fn pbytes(&mut self, src: &[u8], offset: usize, length: usize) {
        let required_len = self.position + length;

        // Ensure we have enough space
        if required_len > self.data.len() {
            self.data.resize(required_len, 0);
        }

        // Copy the bytes
        self.data[self.position..self.position + length].copy_from_slice(&src[offset..offset + length]);
        self.position += length;
    }

    #[inline(always)]
    pub fn g1(&mut self) -> u8 {
        // Bounds check with branch prediction hint
        if likely!(self.position < self.data.len()) {
            let value = self.data[self.position];
            self.position += 1;
            value
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn g1b(&mut self) -> i8 {
        // Bounds check with branch prediction hint
        if likely!(self.position < self.data.len()) {
            let value = self.data[self.position] as i8;
            self.position += 1;
            value
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn g2(&mut self) -> u16 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 1 < self.data.len()) {
            let pos = self.position;
            self.position += 2;
            u16::from_be_bytes([self.data[pos], self.data[pos + 1]])
        } else {
            if self.position < self.data.len() {
                // Handle partial reads
                let result = (self.data[self.position] as u16) << 8;
                self.position = self.data.len();
                result
            } else {
                0
            }
        }
    }

    #[inline(always)]
    pub fn g2s(&mut self) -> i16 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 1 < self.data.len()) {
            let pos = self.position;
            self.position += 2;
            i16::from_be_bytes([self.data[pos], self.data[pos + 1]])
        } else {
            if self.position < self.data.len() {
                // Handle partial reads
                let result = (self.data[self.position] as i16) << 8;
                self.position = self.data.len();
                result
            } else {
                0
            }
        }
    }

    #[inline(always)]
    pub fn ig2s(&mut self) -> i16 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 1 < self.data.len()) {
            let pos = self.position;
            self.position += 2;
            i16::from_le_bytes([self.data[pos], self.data[pos + 1]])
        } else {
            if self.position < self.data.len() {
                // Handle partial reads
                let result = self.data[self.position] as i16;
                self.position = self.data.len();
                result
            } else {
                0
            }
        }
    }

    #[inline(always)]
    pub fn g3(&mut self) -> i32 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 2 < self.data.len()) {
            let pos = self.position;
            self.position += 3;

            ((self.data[pos] as i32) << 16) |
                ((self.data[pos + 1] as i32) << 8) |
                (self.data[pos + 2] as i32)
        } else {
            // Handle partial reads
            let mut result = 0;
            let bytes_available = self.data.len().saturating_sub(self.position);

            if bytes_available > 0 {
                result |= (self.data[self.position] as i32) << 16;
                if bytes_available > 1 {
                    result |= (self.data[self.position + 1] as i32) << 8;
                }
            }

            self.position = self.data.len();
            result
        }
    }

    #[inline(always)]
    pub fn g4(&mut self) -> i32 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 3 < self.data.len()) {
            let pos = self.position;
            self.position += 4;

            ((self.data[pos] as i32) << 24) |
                ((self.data[pos + 1] as i32) << 16) |
                ((self.data[pos + 2] as i32) << 8) |
                (self.data[pos + 3] as i32)
        } else {
            // Handle partial reads
            let mut result = 0;
            let bytes_available = self.data.len().saturating_sub(self.position);

            for i in 0..bytes_available {
                result |= (self.data[self.position + i] as i32) << (8 * (3 - i));
            }

            self.position = self.data.len();
            result
        }
    }

    #[inline(always)]
    pub fn g4s(&mut self) -> i32 {
        self.g4() // They do the same thing
    }

    #[inline(always)]
    pub fn ig4s(&mut self) -> i32 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 3 < self.data.len()) {
            let pos = self.position;
            self.position += 4;
            i32::from_le_bytes([
                self.data[pos],
                self.data[pos + 1],
                self.data[pos + 2],
                self.data[pos + 3]
            ])
        } else {
            // Handle partial reads
            let mut bytes = [0u8; 4];
            let bytes_available = self.data.len().saturating_sub(self.position);

            for i in 0..bytes_available {
                bytes[i] = self.data[self.position + i];
            }

            self.position = self.data.len();
            i32::from_le_bytes(bytes)
        }
    }

    #[inline(always)]
    pub fn g8s(&mut self) -> i64 {
        // Bounds check with branch prediction hint
        if likely!(self.position + 7 < self.data.len()) {
            let pos = self.position;
            self.position += 8;
            i64::from_be_bytes([
                self.data[pos],
                self.data[pos + 1],
                self.data[pos + 2],
                self.data[pos + 3],
                self.data[pos + 4],
                self.data[pos + 5],
                self.data[pos + 6],
                self.data[pos + 7]
            ])
        } else {
            // Handle partial reads
            let mut bytes = [0u8; 8];
            let bytes_available = self.data.len().saturating_sub(self.position);

            for i in 0..bytes_available {
                bytes[i] = self.data[self.position + i];
            }

            self.position = self.data.len();
            i64::from_be_bytes(bytes)
        }
    }

    /// Reads a string from the internal buffer until a terminator byte is encountered.
    #[inline(always)]
    pub fn gjstr(&mut self, terminator: u8) -> String {
        let start_pos = self.position;

        // Find terminator
        while self.position < self.data.len() && self.data[self.position] != terminator {
            self.position += 1;
        }

        // Create string from data
        let result = if start_pos < self.position {
            match std::str::from_utf8(&self.data[start_pos..self.position]) {
                Ok(s) => s.to_owned(),
                Err(_) => String::new(), // Handle invalid UTF-8
            }
        } else {
            String::new()
        };

        // Skip past terminator if we found it
        if self.position < self.data.len() {
            self.position += 1;
        }

        result
    }

    #[inline(always)]
    pub fn gsmart(&mut self) -> i32 {
        if self.position >= self.data.len() {
            return 0;
        }

        if self.data[self.position] < 128 {
            self.g1() as i32
        } else {
            self.g2() as i32 - 32768
        }
    }

    #[inline(always)]
    pub fn gsmarts(&mut self) -> i32 {
        if self.position >= self.data.len() {
            return 0;
        }

        if self.data[self.position] < 128 {
            self.g1() as i32 - 64
        } else {
            self.g2() as i32 - 49152
        }
    }

    #[inline(always)]
    pub fn gbytes(&mut self, length: usize) -> Vec<u8> {
        if self.position >= self.data.len() {
            return Vec::new();
        }
        
        let to_read = std::cmp::min(length, self.remaining() as usize);

        let result = self.data[self.position..self.position + to_read].to_vec();
        self.position += to_read;
        result
    }

    /// Sets the internal bit position (`bit_position`) to the current byte position (`position`)
    /// converted to bit position. This is typically used when switching from byte-based
    /// to bit-based addressing.
    #[inline(always)]
    pub fn bits(&mut self) {
        self.bit_position = self.position << 3;
    }

    /// Sets the internal byte position (`position`) based on the current bit position (`bit_position`).
    /// This is typically used when switching from bit-based addressing back to byte-based addressing.
    #[inline(always)]
    pub fn bytes(&mut self) {
        self.position = (self.bit_position + 7) >> 3;
    }
}