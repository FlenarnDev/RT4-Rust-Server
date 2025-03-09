use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result, AsyncReadExt, AsyncWriteExt, copy};
use std::net::SocketAddr;
use log::{debug, error, info};
use tokio::io;
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

pub async fn write_and_clear_output(conn: &mut Connection) -> Result<()> {
    if !conn.output.data.is_empty() {
        conn.socket.write_all(&conn.output.data).await?;
        conn.socket.flush().await?;
        conn.output.data.clear(); // Clear the output packet after writing
        debug!("Output packet written and cleared");
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
                let mut single_byte = [0; 1];
                conn.socket.read_exact(&mut single_byte).await?;
                let opcode = single_byte[0];
                debug!("Received opcode: {}", opcode);

                match opcode {
                    title_protocol::JS5OPEN => {
                        let mut version_bytes = [0; 4];
                        conn.socket.read_exact(&mut version_bytes).await?;
                        let client_version = u32::from_be_bytes(version_bytes);
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
                        conn.state = ClientState::CONNECTED;
                    }
                }
            }
            ClientState::JS5 => {
                let target_addr = "127.0.0.1:43595".parse::<SocketAddr>().unwrap();
                let target_stream = TcpStream::connect(target_addr).await?;
                conn.state = ClientState::PROXYING(target_stream);
            }
            ClientState::WORLDLIST => {
                let target_addr = "127.0.0.1:43596".parse::<SocketAddr>().unwrap();
                let target_stream = TcpStream::connect(target_addr).await?;
                conn.state = ClientState::PROXYING(target_stream);
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