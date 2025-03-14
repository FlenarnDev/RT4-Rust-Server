use std::io::ErrorKind;
use std::io::{Read, Write};
use constants::proxy::proxy::BUFFER_SIZE;
use std::net::{Shutdown, TcpStream};
use io::client_state::ConnectionState;
use io::isaac::Isaac;
use io::packet::Packet;

pub struct GameClient {
    pub stream: Option<TcpStream>,
    pub inbound: Packet,
    pub outbound: Packet,
    pub state: ConnectionState,
    total_bytes_read: usize,
    total_bytes_written: usize,
    pub(crate) encryptor: Option<Isaac>,
    decryptor: Option<Isaac>,
    /// Current opcode being read.
    pub opcode: i32,
    /// Bytes to wait for (if any)
    pub waiting: i32,
    read_buffer: Vec<u8>,
}

impl GameClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            inbound: Packet::new(BUFFER_SIZE),
            outbound: Packet::new(1),
            state: ConnectionState::New,
            total_bytes_read: 0,
            total_bytes_written: 0,
            encryptor: None,
            decryptor: None,
            opcode: -1,
            waiting: 0,
            read_buffer: vec![0u8; BUFFER_SIZE],
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
            opcode: -1,
            waiting: 0,
            read_buffer: Vec::with_capacity(1),
        }
    }

    /// Read data from stream into inbound packet.
    pub fn read_packet(&mut self) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        self.inbound.position = 0;
        let bytes_read = self.stream.as_mut().unwrap().read(&mut self.read_buffer)?;

        if bytes_read > 0 {
            self.inbound.data.clear();
            self.inbound.data.extend_from_slice(&self.read_buffer[0..bytes_read]);
            self.total_bytes_read += bytes_read;
        }

        Ok(bytes_read)
    }

    /// Read data with specified size from stream into inbound packet.
    pub fn read_packet_with_size(&mut self, size: usize) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        self.inbound.position = 0;

        if self.read_buffer.len() < size {
            self.read_buffer.resize(size, 0);
        }

        let bytes_read = self.stream.as_mut().unwrap().read(&mut self.read_buffer[0..size])?;

        if bytes_read > 0 {
            self.inbound.data.clear();
            self.inbound.data.extend_from_slice(&self.read_buffer[0..bytes_read]);
            self.total_bytes_read += bytes_read;
        }

        Ok(bytes_read)
    }

    /// Write data from outbound packet to stream
    pub fn write_packet(&mut self) -> Result<usize, std::io::Error> {
        if self.stream.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "No connection"));
        }

        let bytes_written = self.stream.as_mut().unwrap().write(&self.outbound.data[0..self.outbound.position])?;
        self.stream.as_mut().unwrap().flush()?;

        // Reset outbound packet for next use
        self.outbound.position = 0;
        self.outbound.data.clear();
        self.total_bytes_written += bytes_written;

        Ok(bytes_written)
    }

    /// Get a reference to the inbound packet (for reading received data)
    pub fn inbound(&mut self) -> &mut Packet {
        &mut self.inbound
    }

    /// Get a reference to the outbound packet (for preparing data to send)
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

    pub fn is_connection_active(&self) -> bool {
        match &self.stream {
            None => false,
            Some(stream) => {
                let mut buf = [0; 1];

                // Try to set non-blocking temporarily
                let was_nonblocking = stream.set_nonblocking(true).is_ok();

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
                if was_nonblocking {
                    let _ = stream.set_nonblocking(false);
                }

                result
            }
        }
    }

    // Take ownership of a connection
    pub fn take_ownership(connection: &mut Option<GameClient>) -> GameClient {
        connection.take().unwrap_or_else(|| GameClient::new_dummy())
    }

    // Optimized method to take just the stream from a connection
    pub fn take_stream(&mut self) -> Option<TcpStream> {
        self.stream.take()
    }

    // Add a stream to a dummy connection
    pub fn with_stream(mut self, stream: TcpStream) -> Self {
        self.stream = Some(stream);
        self.state = ConnectionState::New;
        self
    }
}