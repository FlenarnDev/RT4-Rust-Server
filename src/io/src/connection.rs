use std::io::ErrorKind;
use tokio::net::TcpStream;
use tokio::io::{Result, AsyncReadExt, AsyncWriteExt, copy};
use std::net::SocketAddr;
use log::{debug, error, info, warn};
use constants::js5_out::js5_out;
use constants::title_protocol::title_protocol;
use crate::packet::Packet;
use crate::client_state::ClientState;


pub struct Connection {
    pub socket: TcpStream,
    pub state: ClientState,
    pub input: Packet,
    pub output: Packet,
    pub active: bool,
    pub peer_addr: SocketAddr,
}

/// Writes the output packet data of the connection to the socket and clears the packet.
///
/// This function checks if the output packet contains any data. If it does, it writes the data to the
/// connection's socket, flushes the socket to ensure the data is sent immediately, and then clears
/// the packet's data. This helps to reset the output packet for further use after the data is sent.
///
/// Logs a debug message after successfully writing and clearing the output.
///
/// # Arguments
/// - `conn`: A mutable reference to the `Connection` whose output packet needs to be written.
///
/// # Returns
/// A Result indicating success or failure of the I/O operations.
pub async fn write_and_clear_output(conn: &mut Connection) -> Result<()> {
    if !conn.output.data.is_empty() {
        conn.socket.write_all(&conn.output.data).await?;
        conn.socket.flush().await?;
        conn.output.data.clear(); // Clear the output packet after writing
    }
    Ok(())
}

pub async fn handle_connection(mut conn: Connection) -> Result<()> {
    debug!("Handling connection from {}", conn.peer_addr);

    loop {
        match conn.state {
            ClientState::CONNECTED => {
                // Read only single byte from the socket, we need nothing more to
                // identify the title protocol. 
                // Over-reading breaks the worldlist fetching.
                let opcode = conn.socket.read_u8().await?;
                debug!("Received opcode: {}", opcode);

                match opcode {
                    title_protocol::JS5OPEN => {
                        let client_version = conn.socket.read_u32().await?;
                        debug!("Client version: {}", client_version);
                        
                        if client_version == 530 {
                            conn.output.p1(js5_out::SUCCESS);
                            conn.state = ClientState::JS5;
                        } else {
                            conn.output.p1(js5_out::OUT_OF_DATE);
                            conn.state = ClientState::CLOSED;
                        }
                        write_and_clear_output(&mut conn).await?;
                    }
                    title_protocol::WORLDLIST_FETCH => {
                        conn.state = ClientState::WORLDLIST;
                    }
                    _ => {
                        debug!("Unknown opcode: {}", opcode);
                        conn.state = ClientState::CLOSED;
                    }
                }
            }
            ClientState::JS5 => {
                let target_addr = "127.0.0.1:43595".parse::<SocketAddr>().unwrap();
                match TcpStream::connect(target_addr).await {
                    Ok(target_stream) => {
                        target_stream.set_nodelay(true)?;
                        conn.state = ClientState::PROXYING(target_stream);
                    },
                    Err(e) => {
                        debug!("Failed to connect to JS5 server: {:?}", e);
                        conn.state = ClientState::CLOSED;
                    }
                }
            }
            ClientState::WORLDLIST => {
                let target_addr = "127.0.0.1:43596".parse::<SocketAddr>().unwrap();
                match TcpStream::connect(target_addr).await {
                    Ok(target_stream) => {
                        target_stream.set_nodelay(true)?;
                        conn.state = ClientState::PROXYING(target_stream);
                    },
                    Err(e) => {
                        debug!("Failed to connect to worldlist server: {:?}", e);
                        conn.state = ClientState::CLOSED;
                    }
                }
            }

            ClientState::PROXYING(ref mut target_stream) => {
                let (mut ri, mut wi) = conn.socket.split();
                let (mut ro, mut wo) = target_stream.split();

                tokio::select! {
                    result1 = copy(&mut ri, &mut wo) => {
                        if result1.is_err() {
                            conn.state = ClientState::CLOSED;
                            debug!("Error while proxying: {:?}", result1);
                            break;
                        }
                    },
                    result2 = copy(&mut ro, &mut wi) => {
                        if result2.is_err() {
                            conn.state = ClientState::CLOSED;
                            debug!("Error while proxying: {:?}", result2);
                            break;
                        }
                    },
                }
            }
            ClientState::CLOSED => {
                debug!("Closing connection from {}", conn.peer_addr);
                conn.socket.shutdown().await?;
                conn.active = false;
                break;
            }

            _ => {
                conn.state = ClientState::CLOSED;
            }
        }

        if !conn.active {
            break;
        }
    }
    Ok(())
}