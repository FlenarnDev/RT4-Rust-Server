use std::io::ErrorKind;
use std::io::{Read, Write};
use constants::proxy::proxy::BUFFER_SIZE;
use std::net::{Shutdown, TcpStream};
use crate::io::client_state::ConnectionState;
use crate::io::isaac::Isaac;
use crate::io::packet::Packet;

#[derive(Debug)]
pub struct GameClient {
    pub stream: Option<TcpStream>,
    pub inbound: Packet,
    pub outbound: Packet,
    pub state: ConnectionState,
    total_bytes_read: usize,
    total_bytes_written: usize,
    pub encryptor: Option<Isaac>,
    pub decryptor: Option<Isaac>,
    /// Current opcode being read.
    pub opcode: u8,
    /// Bytes to wait for (if any)
    pub waiting: i32,
    read_buffer: Vec<u8>,
    // Track nonblocking state to avoid toggling
    nonblocking: bool,
}

impl Clone for GameClient {
    fn clone(&self) -> Self {
        GameClient {
            // For TcpStream, we can't clone it, so we'll use None
            stream: None,  // The clone will need to re-establish connection
            inbound: self.inbound.clone(),
            outbound: self.outbound.clone(),
            state: self.state.clone(),
            total_bytes_read: self.total_bytes_read,
            total_bytes_written: self.total_bytes_written,
            encryptor: self.encryptor.clone(),
            decryptor: self.decryptor.clone(),
            opcode: self.opcode,
            waiting: self.waiting,
            read_buffer: self.read_buffer.clone(),
            nonblocking: false,
        }
    }
}

impl PartialEq for GameClient {
    /// Since [TcpStream] isn't built to handle full comparisons we can't derive 'PartialEq', we will eventually handle
    /// parts of it in comparisons.
    fn eq(&self, other: &Self) -> bool {
        self.inbound == other.inbound &&
            self.outbound == other.outbound &&
            self.state == other.state &&
            self.total_bytes_read == other.total_bytes_read &&
            self.total_bytes_written == other.total_bytes_written &&
            self.encryptor == other.encryptor &&
            self.decryptor == other.decryptor &&
            self.opcode == other.opcode &&
            self.waiting == other.waiting &&
            self.read_buffer == other.read_buffer
    }
}

impl GameClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            inbound: Packet::new(BUFFER_SIZE),
            outbound: Packet::new(BUFFER_SIZE), // Increase initial size from 1 to BUFFER_SIZE
            state: ConnectionState::New,
            total_bytes_read: 0,
            total_bytes_written: 0,
            encryptor: None,
            decryptor: None,
            opcode: 0,
            waiting: 0,
            read_buffer: vec![0u8; BUFFER_SIZE],
            nonblocking: false,
        }
    }

    pub fn new_dummy() -> Self {
        Self {
            stream: None,
            inbound: Packet::new(1),
            outbound: Packet::new(1),
            state: ConnectionState::Null,
            total_bytes_read: 0,
            total_bytes_written: 0,
            encryptor: None,
            decryptor: None,
            opcode: 0,
            waiting: 0,
            read_buffer: Vec::with_capacity(1),
            nonblocking: false,
        }
    }

    /// Read data from stream into inbound packet.
    #[inline]
    pub fn read_packet(&mut self) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        self.inbound.position = 0;

        // Directly read into the buffer
        let bytes_read = self.stream.as_mut().unwrap().read(&mut self.read_buffer)?;

        if bytes_read > 0 {
            // Clear existing data and ensure capacity
            self.inbound.data.clear();
            self.inbound.data.reserve(bytes_read);

            // Use unsafe for performance in critical section
            unsafe {
                // Set length to match read bytes and copy directly
                self.inbound.data.set_len(bytes_read);
                std::ptr::copy_nonoverlapping(
                    self.read_buffer.as_ptr(),
                    self.inbound.data.as_mut_ptr(),
                    bytes_read
                );
            }

            self.total_bytes_read += bytes_read;
        }

        Ok(bytes_read)
    }

    /// Read data with specified size from stream into inbound packet.
    #[inline]
    pub fn read_packet_with_size(&mut self, size: usize) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        self.inbound.position = 0;

        // Ensure buffer is large enough
        if self.read_buffer.len() < size {
            self.read_buffer.resize(size, 0);
        }

        // Read exact number of bytes if possible
        match self.stream.as_mut().unwrap().read_exact(&mut self.read_buffer[0..size]) {
            Ok(_) => {
                // Clear and ensure capacity for inbound data
                self.inbound.data.clear();
                self.inbound.data.reserve(size);

                // Use unsafe for performance in critical section
                unsafe {
                    // Set length to match size and copy directly
                    self.inbound.data.set_len(size);
                    std::ptr::copy_nonoverlapping(
                        self.read_buffer.as_ptr(),
                        self.inbound.data.as_mut_ptr(),
                        size
                    );
                }

                self.total_bytes_read += size;
                Ok(size)
            },
            Err(e) => {
                // Handle partial reads
                if e.kind() == ErrorKind::UnexpectedEof {
                    // Try a normal read for whatever bytes are available
                    let bytes_read = self.stream.as_mut().unwrap().read(&mut self.read_buffer[0..size])?;

                    if bytes_read > 0 {
                        self.inbound.data.clear();
                        self.inbound.data.reserve(bytes_read);

                        unsafe {
                            self.inbound.data.set_len(bytes_read);
                            std::ptr::copy_nonoverlapping(
                                self.read_buffer.as_ptr(),
                                self.inbound.data.as_mut_ptr(),
                                bytes_read
                            );
                        }

                        self.total_bytes_read += bytes_read;
                    }

                    Ok(bytes_read)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Write data from outbound [Packet] to stream
    #[inline]
    pub fn write_packet(&mut self) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        // Skip if nothing to write
        if self.outbound.position == 0 {
            return Ok(0);
        }

        // Use write_all for more reliable writing
        let bytes_to_write = self.outbound.position;
        self.stream.as_mut().unwrap().write_all(&self.outbound.data[0..bytes_to_write])?;
        self.stream.as_mut().unwrap().flush()?;

        // Reset outbound packet for next use but maintain capacity
        self.outbound.position = 0;
        self.outbound.data.clear();
        self.total_bytes_written += bytes_to_write;

        Ok(bytes_to_write)
    }

    /// Get a reference to the inbound packet (for reading received data)
    #[inline]
    pub fn inbound(&mut self) -> &mut Packet {
        &mut self.inbound
    }

    /// Get a reference to the outbound packet (for preparing data to send)
    #[inline]
    pub fn outbound(&mut self) -> &mut Packet {
        &mut self.outbound
    }

    pub fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        match &self.stream {
            Some(stream) => stream.peer_addr(),
            None => Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"))
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(stream) = &self.stream {
            let _ = stream.shutdown(Shutdown::Both);
        }
        self.stream = None;
    }

    #[inline]
    pub fn is_connection_active(&self) -> bool {
        match &self.stream {
            None => false,
            Some(stream) => {
                let mut buf = [0; 1];

                // Only toggle nonblocking if needed
                let need_toggle = !self.nonblocking;
                let mut was_toggled = false;

                // Set nonblocking temporarily if needed
                if need_toggle {
                    was_toggled = stream.set_nonblocking(true).is_ok();
                }

                let result = match stream.peek(&mut buf) {
                    Ok(0) => false, // Connection closed (EOF)
                    Ok(_) => true,  // Data available
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::WouldBlock => true, // No data but connection is open
                            ErrorKind::ConnectionReset |
                            ErrorKind::ConnectionAborted |
                            ErrorKind::BrokenPipe => false,
                            _ => true, // Other errors might be temporary
                        }
                    }
                };

                // Restore original blocking mode if needed
                if need_toggle && was_toggled {
                    let _ = stream.set_nonblocking(false);
                }

                result
            }
        }
    }

    // Take ownership of a connection
    #[inline]
    pub fn take_ownership(connection: &mut Option<GameClient>) -> GameClient {
        connection.take().unwrap_or_else(|| GameClient::new_dummy())
    }

    // Optimized method to take just the stream from a connection
    #[inline]
    pub fn take_stream(&mut self) -> Option<TcpStream> {
        self.stream.take()
    }

    // Add a stream to a dummy connection
    #[inline]
    pub fn with_stream(mut self, stream: TcpStream) -> Self {
        self.stream = Some(stream);
        self.state = ConnectionState::New;
        self
    }

    #[inline]
    pub fn has_available(&mut self, required_bytes: usize) -> Result<bool, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        // Cache stream reference
        let stream = self.stream.as_ref().unwrap();

        // Only toggle nonblocking if needed (caching state)
        let was_nonblocking;
        if !self.nonblocking {
            was_nonblocking = false;
            match stream.set_nonblocking(true) {
                Ok(_) => (),
                Err(_) => return Err(std::io::Error::new(ErrorKind::Other, "Failed to set nonblocking")),
            }
        } else {
            was_nonblocking = true;
        }

        // Use a stack-allocated buffer for small peek requests
        let mut stack_buf = [0u8; 16];
        let peek_result = if required_bytes <= stack_buf.len() {
            // Use stack buffer for small requests
            stream.peek(&mut stack_buf[0..required_bytes])
        } else {
            // For larger requests, ensure our read buffer is large enough
            if self.read_buffer.len() < required_bytes {
                self.read_buffer.resize(required_bytes, 0);
            }
            stream.peek(&mut self.read_buffer[0..required_bytes])
        };

        // Process the peek result
        let result = match peek_result {
            Ok(n) => n >= required_bytes, // Return true if we have enough bytes
            Err(e) if e.kind() == ErrorKind::WouldBlock => false, // No data available
            Err(e) => {
                // Restore blocking state if needed
                if !was_nonblocking {
                    let _ = stream.set_nonblocking(false);
                    self.nonblocking = false;
                }
                return Err(e);
            }
        };

        // Restore blocking state if needed
        if !was_nonblocking {
            let _ = stream.set_nonblocking(false);
            self.nonblocking = false;
        }

        Ok(result)
    }

    // Method to toggle and maintain nonblocking state
    pub fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        // Only change if state is different
        if self.nonblocking != nonblocking {
            self.stream.as_ref().unwrap().set_nonblocking(nonblocking)?;
            self.nonblocking = nonblocking;
        }

        Ok(())
    }

    // Batch write multiple buffers to reduce system calls
    pub fn write_multiple(&mut self, buffers: &[&[u8]]) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        let mut total_written = 0;

        // Combine small buffers into a single write
        for buffer in buffers {
            // Add to outbound buffer
            let start_pos = self.outbound.position;
            let buffer_len = buffer.len();
            let end_pos = start_pos + buffer_len;

            // Ensure capacity
            if end_pos > self.outbound.data.len() {
                self.outbound.data.resize(end_pos, 0);
            }

            // Copy buffer
            self.outbound.data[start_pos..end_pos].copy_from_slice(buffer);
            self.outbound.position = end_pos;

            total_written += buffer_len;
        }

        // Write combined data
        self.write_packet()?;

        Ok(total_written)
    }

    // Directly write a buffer without copying to outbound first (for large buffers)
    pub fn write_direct(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        // For large buffers, bypass the outbound packet
        if buffer.len() > BUFFER_SIZE {
            let bytes_written = self.stream.as_mut().unwrap().write(buffer)?;
            self.total_bytes_written += bytes_written;
            return Ok(bytes_written);
        }

        // For smaller buffers, use normal path
        self.outbound.data.clear();
        self.outbound.data.extend_from_slice(buffer);
        self.outbound.position = buffer.len();

        self.write_packet()
    }
}