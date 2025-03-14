use std::io::ErrorKind;
use std::io::{Read, Write};
use constants::proxy::proxy::BUFFER_SIZE;
use std::net::TcpStream;
use uuid::Uuid;
use io::client_state::ConnectionState;
use io::isaac::Isaac;
use io::packet::Packet;
use crate::entity::network_player::NetworkPlayer;

pub struct GameConnection {
    pub stream: TcpStream,
    pub inbound: Packet,
    pub outbound: Packet,
    pub state: ConnectionState,
    uuid: Uuid,
    total_bytes_read: usize,
    total_bytes_written: usize,
    player: Option<NetworkPlayer>,
    encryptor: Option<Isaac>,
    decryptor: Option<Isaac>,
    /// Current opcode being read.
    pub opcode: i32,
    /// Bytes to wait for (if any)
    pub waiting: i32,
}

impl GameConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            inbound: Packet::new(BUFFER_SIZE),
            outbound: Packet::new(1),
            state: ConnectionState::New,
            uuid: Uuid::new_v4(),
            total_bytes_read: 0,
            total_bytes_written: 0,
            player: None,
            encryptor: None,
            decryptor: None,
            opcode: -1,
            waiting: 0,
        }
    }
    
    /// Read data from stream into inbound packet.
    pub fn read_packet(&mut self) -> Result<usize, std::io::Error> {
        self.inbound.position = 0;
        
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let bytes_read = self.stream.read(&mut buffer)?;
        
        if bytes_read > 0 {
            self.inbound.data.clear();
            self.inbound.data.extend_from_slice(&buffer[0..bytes_read]);
        }
        
        Ok(bytes_read)
    }
    
    /// Write data from outbound packet to stream
    pub fn write_packet(&mut self) -> Result<usize, std::io::Error> {
        let bytes_written = self.stream.write(&self.outbound.data[0..self.outbound.position])?;
        self.stream.flush()?;
        
        // Reset outbound packet for next use
        self.outbound.position = 0;
        self.outbound.data.clear();
        
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
        self.stream.peer_addr()
    }
    
    pub fn shutdown(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both).expect("Failed to shutdown connection");
    }
    
    pub fn is_connection_active(&self) -> bool {
        let current_blocking_mode = self.stream.set_nonblocking(true).is_ok();

        let mut buf = [0; 1];
        let result = match self.stream.peek(&mut buf) {
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

        // Restore original blocking mode
        if current_blocking_mode {
            let _ = self.stream.set_nonblocking(false);
        }

        result
    }
}