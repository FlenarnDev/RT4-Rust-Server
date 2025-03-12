mod js5_request;

use std::error::Error;
use cache::file_handler::ensure_initialized;
use constants::js5_out::js5_out;
use constants::server_addresses::server_addresses::JS5_ADDR;
use io::client_state::ConnectionState;
use io::connection::Connection;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use constants::js5_in::js5_in;
use crate::js5_request::Js5Request;

async fn run_js5_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(JS5_ADDR).await?;

    debug!("Initializing cache in main thread");
    if let Err(e) = ensure_initialized() {
        error!("Failed to initialize cache in main thread: {}", e);
    } else {
        debug!("Cache successfully initialized in main thread");
    }

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_js5_client(stream).await {
                        error!("Connection handler error: {}", e);
                    }
                });
            },
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
}

async fn handle_js5_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let addr = stream.peer_addr()?;
    info!("New connection from: {}", addr);
    
    let mut connection = Connection::new(stream);
    
    loop {
        match connection.read_packet().await {
            Ok(0) => {
                debug!("Connection closed by client: {}", addr);
                break;
            },
            Ok(n) => {
                debug!("Received packet: {} bytes from: {}", n, addr);
                if connection.state == ConnectionState::New {
                    let client_version = if !connection.inbound().is_empty() {
                        connection.inbound().g4()
                    } else {
                        0
                    };

                    debug!("Client version is {}", client_version);
                    if client_version == 530 {
                        connection.outbound().p1(js5_out::SUCCESS);
                        connection.state = ConnectionState::Connected;

                    } else {
                        connection.outbound().p1(js5_out::OUT_OF_DATE);
                        connection.state = ConnectionState::Closed;

                    }
                } else if connection.state == ConnectionState::Connected {
                    while connection.inbound().remaining() > 0 {
                        let opcode = connection.inbound().g1();
                        let request = match opcode {
                            js5_in::PREFETCH | js5_in::URGENT => {
                                Js5Request::Group {
                                    urgent: opcode == js5_in::PREFETCH,
                                    archive: connection.inbound.g1(),
                                    group: connection.inbound.g2()
                                }
                            }

                            js5_in::REKEY => {
                                let key = connection.inbound.g1();
                                if connection.inbound.g2() != 0 {
                                    Js5Request::Invalid
                                } else {
                                    Js5Request::Rekey { key }
                                }
                            }
                            js5_in::LOGGED_IN => {
                                if connection.inbound.g3() != 0 {
                                    Js5Request::Invalid
                                } else {
                                    Js5Request::LoggedIn
                                }
                            }
                            js5_in::LOGGED_OUT => {
                                connection.inbound().g3();
                                Js5Request::LoggedOut
                            }
                            js5_in::CONNECTED => {
                                // Value is always '3'.
                                if connection.inbound.g3() != 3 {
                                    Js5Request::Invalid
                                } else {
                                    Js5Request::Connected
                                }
                            }
                            js5_in::DISCONNECT => {
                                connection.inbound().g3();
                                Js5Request::Disconnect
                            }
                            _ => {
                                debug!("Invalid opcode: {}", opcode);
                                connection.inbound().g3();
                                Js5Request::Invalid
                            }
                        };

                        match request {
                            Js5Request::Group { .. } => {
                                Js5Request::fulfill_request(&mut connection, &request)?;
                            }

                            Js5Request::Invalid => {
                                // TODO - terminate early.
                            }
                            _ => {
                                // Currently nothing.
                            }
                        }
                    }
                } else {
                    error!("Client state is undefined.");
                    connection.state = ConnectionState::Closed;
                }

                // Send response if outbound isn't empty
                if !connection.outbound.is_empty() {
                    match connection.write_packet().await {
                        Ok(bytes_written) => {
                            debug!("Sent response packet: {} bytes", bytes_written);
                        },
                        Err(e) => {
                            error!("Error writing to client: {}", e);
                            connection.state = ConnectionState::Closed;
                            break;
                        }
                    }
                }

                if connection.state == ConnectionState::Closed {
                    connection.shutdown().await?;
                    break;
                }
            },
            Err(e) => {
                error!("Error reading from client: {}", e);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    info!("Starting JS5 System");
    info!("---------------------------------------------");
    info!("Starting JS5 server: {}", JS5_ADDR);
    info!("---------------------------------------------");

    tokio::select! {
        result = run_js5_server() => {
            if let Err(e) = result {
                error!("JS5 server error: {}", e);
            }
        }
    }

    Ok(())
}