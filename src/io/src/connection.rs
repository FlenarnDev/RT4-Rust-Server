use log::{debug, error};
use constants::proxy::proxy::BUFFER_SIZE;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::client_state::ConnectionState;
use crate::packet::Packet;

pub struct Connection {
    pub stream: TcpStream,
    pub inbound: Packet,
    pub outbound: Packet,
    pub state: ConnectionState,
}

impl Connection {
    /// Create a new connection with default packet sizes
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            inbound: Packet::new(BUFFER_SIZE),
            outbound: Packet::new(BUFFER_SIZE),
            state: ConnectionState::New,
        }
    }

    /// Read data from stream into inbound packet
    pub async fn read_packet(&mut self) -> Result<usize, std::io::Error> {
        self.inbound.position = 0;

        let mut buffer = vec![0u8; BUFFER_SIZE];
        let bytes_read = self.stream.read(&mut buffer).await?;

        if bytes_read > 0 {
            self.inbound.data.clear();
            self.inbound.data.extend_from_slice(&buffer[0..bytes_read]);
        }

        Ok(bytes_read)
    }

    /// Write data from outbound packet to stream
    pub async fn write_packet(&mut self) -> Result<usize, std::io::Error> {
        let bytes_written = self.stream.write(&self.outbound.data[0..self.outbound.position]).await?;
        self.stream.flush().await?;  // Ensure data is sent immediately

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

    /// Get peer address
    pub fn peer_addr(&self) -> Result<std::net::SocketAddr, std::io::Error> {
        self.stream.peer_addr()
    }

    /// Shutdown the connection
    pub async fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown().await
    }

    /// Create a packet directly from initial data
    pub fn from_initial_data(stream: TcpStream, initial_data: Vec<u8>) -> Self {
        let mut connection = Self::new(stream);
        connection.inbound.data = initial_data;
        connection
    }
}

pub async fn try_write_packet(connection: &mut Connection) {
    if !connection.outbound.is_empty() {
        match connection.write_packet().await {
            Ok(bytes_written) => {
                debug!("Sent response packet: {} bytes", bytes_written);

            },
            Err(e) => {
                error!("Error writing to client: {}", e);
                connection.state = ConnectionState::Closed;

            }
        }
    }
}