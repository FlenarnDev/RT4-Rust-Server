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
            data: vec![0; size], // Initialize with zeros instead of just capacity
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
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the packet contains no data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline(always)]
    pub fn p1(&mut self, value: i32) {
        let value_byte = value as u8;
        if self.position >= self.data.len() {
            self.data.push(value_byte);
        } else {
            self.data[self.position] = value_byte;
        }
        self.position += 1;
    }

    #[inline(always)]
    pub fn p2(&mut self, value: i32) {
        let required_len = self.position + 2;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start..start + 2].copy_from_slice(&(value as u16).to_be_bytes());
        self.position += 2;
    }

    #[inline(always)]
    pub fn ip2(&mut self, value: i32) {
        let required_len = self.position + 2;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start..start + 2].copy_from_slice(&(value as u16).to_le_bytes());
        self.position += 2;
    }

    #[inline(always)]
    pub fn p3(&mut self, value: i32) {
        let required_len = self.position + 3;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start] = (value >> 16) as u8;
        self.data[start + 1..start + 3].copy_from_slice(&(value as u16).to_be_bytes());
        self.position += 3;
    }

    #[inline(always)]
    pub fn p4(&mut self, value: i32) {
        let required_len = self.position + 4;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start..start + 4].copy_from_slice(&value.to_be_bytes());
        self.position += 4;
    }

    #[inline(always)]
    pub fn ip4(&mut self, value: i32) {
        let required_len = self.position + 4;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start..start + 4].copy_from_slice(&value.to_le_bytes());
        self.position += 4;
    }

    #[inline(always)]
    pub fn p8(&mut self, value: i64) {
        let required_len = self.position + 8;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start: usize = self.position;
        self.data[start..start + 8].copy_from_slice(&value.to_be_bytes());
        self.position += 8;
    }

    #[inline(always)]
    pub fn pjstr(&mut self, str: &str, terminator: u8) {
        let required_len = self.position + str.len() + 1;
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start = self.position;
        let end = start + str.len();
        self.data[start..end].copy_from_slice(str.as_bytes());
        self.data[end] = terminator;
        self.position = end + 1;
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
        if self.data.len() < required_len {
            self.data.resize(required_len, 0);
        }

        let start = self.position;
        let end = start + length;
        self.data[start..end].copy_from_slice(&src[offset..offset + length]);
        self.position += length;
    }

    #[inline(always)]
    pub fn g1(&mut self) -> u8 {
        if self.position >= self.data.len() {
            return 0; // Prevent out-of-bounds access
        }
        let value = self.data[self.position];
        self.position += 1;
        value
    }

    #[inline(always)]
    pub fn g1b(&mut self) -> i8 {
        if self.position >= self.data.len() {
            return 0; // Prevent out-of-bounds access
        }
        let value = self.data[self.position] as i8;
        self.position += 1;
        value
    }

    #[inline(always)]
    pub fn g2(&mut self) -> u16 {
        if self.position + 2 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 2;
        u16::from_be_bytes(self.data[pos..pos + 2].try_into().unwrap())
    }

    #[inline(always)]
    pub fn g2s(&mut self) -> i16 {
        if self.position + 2 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 2;
        i16::from_be_bytes(self.data[pos..pos + 2].try_into().unwrap())
    }

    #[inline(always)]
    pub fn ig2s(&mut self) -> i16 {
        if self.position + 2 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 2;
        i16::from_le_bytes(self.data[pos..pos + 2].try_into().unwrap())
    }

    #[inline(always)]
    pub fn g3(&mut self) -> i32 {
        if self.position + 3 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 3;

        ((self.data[pos] as u32) << 16 | u16::from_be_bytes(
            self.data[pos + 1..pos + 3].try_into().unwrap()
        ) as u32) as i32
    }

    #[inline(always)]
    pub fn g4(&mut self) -> i32 {
        if self.position + 4 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 4;
        i32::from_be_bytes(self.data[pos..pos + 4].try_into().unwrap())
    }

    #[inline(always)]
    pub fn g4s(&mut self) -> i32 {
        if self.position + 4 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 4;
        i32::from_be_bytes(self.data[pos..pos + 4].try_into().unwrap())
    }

    #[inline(always)]
    pub fn ig4s(&mut self) -> i32 {
        if self.position + 4 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 4;
        i32::from_le_bytes(self.data[pos..pos + 4].try_into().unwrap())
    }

    #[inline(always)]
    pub fn g8s(&mut self) -> i64 {
        if self.position + 8 > self.data.len() {
            self.position = self.data.len();
            return 0; // Prevent out-of-bounds access
        }

        let pos = self.position;
        self.position += 8;
        i64::from_be_bytes(self.data[pos..pos + 8].try_into().unwrap())
    }

    /// Reads a string from the internal buffer until a terminator byte is encountered.
    #[inline(always)]
    pub fn gjstr(&mut self, terminator: u8) -> String {
        let pos = self.position;
        let mut length = pos;

        while length < self.data.len() && self.data[length] != terminator {
            length += 1;
        }

        if length >= self.data.len() {
            self.position = self.data.len();
            return String::new();
        }

        let result = match std::str::from_utf8(&self.data[pos..length]) {
            Ok(s) => s.to_owned(),
            Err(_) => String::new(), // Handle invalid UTF-8
        };

        self.position = length + 1;
        result
    }

    #[inline(always)]
    pub fn gsmart(&mut self) -> i32 {
        if self.position >= self.data.len() {
            return 0; // Prevent out-of-bounds access
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
            return 0; // Prevent out-of-bounds access
        }

        if self.data[self.position] < 128 {
            self.g1() as i32 - 64
        } else {
            self.g2() as i32 - 49152
        }
    }

    #[inline(always)]
    pub fn gbytes(&mut self, length: usize) -> Vec<u8> {
        if self.position + length > self.data.len() {
            let available = if self.position < self.data.len() {
                self.data.len() - self.position
            } else {
                0
            };

            let result = if available > 0 {
                self.data[self.position..self.position + available].to_vec()
            } else {
                Vec::new()
            };

            self.position = self.data.len();
            return result;
        }

        let result = self.data[self.position..self.position + length].to_vec();
        self.position += length;
        result
    }

    /// Sets the internal bit position (`bit_position`) to the current byte position (`position`)
    /// converted to bit position. This is typically used when switching from byte-based
    /// to bit-based addressing.
    #[inline(always)]
    pub fn bits(&mut self) {
        self.bit_position = self.position * 8; // Clearer than << 3
    }

    /// Sets the internal byte position (`position`) based on the current bit position (`bit_position`).
    /// This is typically used when switching from bit-based addressing back to byte-based addressing.
    #[inline(always)]
    pub fn bytes(&mut self) {
        self.position = (self.bit_position + 7) / 8; // Clearer than >> 3
    }
}