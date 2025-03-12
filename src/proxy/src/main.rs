use std::error::Error;
use std::time::Duration;
use constants::proxy::proxy::{BUFFER_SIZE, READ_TIMEOUT_MS};
use constants::server_addresses::server_addresses::{JS5_ADDR, WORLDLIST_ADDR, PROXY_ADDR};
use constants::title_protocol::title_protocol;
use io::connection::Connection;
use io::packet::Packet;
use tokio::net::{TcpListener, TcpStream};
use log::{debug, error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;

#[derive(Debug)]
enum Destination {
    JS5,
    WorldList,
    Terminate,
}

fn choose_backend_and_consume(packet: &mut Packet) -> Destination {
    if packet.is_empty() {
        debug!("Empty packet received, will terminate connection");
        return Destination::Terminate;
    }

    match packet.g1() {
        title_protocol::JS5OPEN => {
            debug!("Routing to JS5_ADDR: {}", JS5_ADDR);
            Destination::JS5
        }

        title_protocol::WORLDLIST_FETCH => {
            debug!("Routing to WORLDLIST_ADDR: {}", WORLDLIST_ADDR);
            Destination::WorldList
        }

        _ => {
            debug!("Unknown packet type, will terminate connection");
            Destination::Terminate
        }
    }
}

/// Helper function to get the address for a destination
fn get_address(destination: &Destination) -> &str {
    match destination {
        Destination::JS5 => JS5_ADDR,
        Destination::WorldList => WORLDLIST_ADDR,
        Destination::Terminate => unreachable!(), // This should never be called
    }
}

async fn handle_proxy_client(client_stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let client_addr = client_stream.peer_addr()?;
    debug!("New connection from: {}", client_addr);

    let mut client_conn = Connection::new(client_stream);

    // Use timeout to avoid waiting indefinitely for initial data
    let read_result = timeout(
        Duration::from_millis(READ_TIMEOUT_MS),
        client_conn.read_packet()
    ).await;

    let read_bytes = match read_result {
        Ok(Ok(n)) => {
            debug!("Read {} bytes of initial data", n);
            if n == 0 {
                debug!("Client closed connection immediately");
                return Ok(());
            }
            n
        },
        Ok(Err(e)) => {
            error!("Error reading from client: {}", e);
            return Ok(());
        },
        Err(_) => {
            error!("Timeout waiting for initial data from {}", client_addr);
            return Ok(());
        }
    };

    // Determine the destination based on the first byte and consume it
    let destination = choose_backend_and_consume(client_conn.inbound());

    // Check if we should terminate
    if matches!(destination, Destination::Terminate) {
        debug!("No valid destination for {}, terminating connection", client_addr);
        return Ok(());
    }

    // Get the backend address for the destination
    let backend_addr = get_address(&destination);
    debug!("Routing client {} to backend: {}", client_addr, backend_addr);

    // Connect to the chosen backend
    let backend_stream = match TcpStream::connect(backend_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect to backend {}: {}", backend_addr, e);
            return Ok(());
        }
    };

    // Extract the data we need from client_conn BEFORE taking ownership of the stream
    let inbound_position = client_conn.inbound().position;
    let initial_data = client_conn.inbound().data[inbound_position..read_bytes].to_vec();

    // Extract the stream from client_conn
    let client_stream = client_conn.stream;

    // Extract the stream from client_conn
    let _ = client_conn;

    // Split the streams to avoid sharing mutable references across tasks
    let (mut client_read, mut client_write) = client_stream.into_split();
    let (mut backend_read, mut backend_write) = backend_stream.into_split();

    // Forward initial data from client to backend
    if let Err(e) = backend_write.write_all(&initial_data).await {
        error!("Error forwarding initial data: {}", e);
        return Ok(());
    }
    debug!("Forwarded {} bytes of data to backend", initial_data.len() - 1);

    // Now we can use more efficient split streams for bidirectional forwarding
    // This avoids the need for mutexes entirely
    let client_to_backend = tokio::spawn(async move {
        let mut buffer = [0u8; BUFFER_SIZE];

        loop {
            match client_read.read(&mut buffer).await {
                Ok(0) => {
                    debug!("Client closed connection");
                    break;
                },
                Ok(n) => {
                    debug!("Read {} bytes from client", n);
                    if let Err(e) = backend_write.write_all(&buffer[0..n]).await {
                        error!("Error writing to backend: {}", e);
                        break;
                    }
                    debug!("Forwarded {} bytes to backend", n);
                },
                Err(e) => {
                    error!("Error reading from client: {}", e);
                    break;
                }
            }
        }

        // Shutdown the write half
        let _ = backend_write.shutdown().await;
    });

    let backend_to_client = tokio::spawn(async move {
        let mut buffer = [0u8; BUFFER_SIZE];

        loop {
            match backend_read.read(&mut buffer).await {
                Ok(0) => {
                    debug!("Backend closed connection");
                    break;
                },
                Ok(n) => {
                    debug!("Read {} bytes from backend", n);
                    if let Err(e) = client_write.write_all(&buffer[0..n]).await {
                        error!("Error writing to client: {}", e);
                        break;
                    }
                    debug!("Forwarded {} bytes to client", n);
                },
                Err(e) => {
                    error!("Error reading from backend: {}", e);
                    break;
                }
            }
        }

        // Shutdown the write half
        let _ = client_write.shutdown().await;
    });

    // Wait for either task to complete
    tokio::select! {
        _ = client_to_backend => debug!("Client to backend task completed"),
        _ = backend_to_client => debug!("Backend to client task completed"),
    }

    debug!("Connection from {} closed", client_addr);
    Ok(())
}

async fn run_proxy_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(PROXY_ADDR).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_proxy_client(stream).await {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }
    env_logger::init();

    info!("Starting TCP Proxy System");
    info!("---------------------------------------------");
    info!("Starting proxy server: {}", PROXY_ADDR);
    info!("---------------------------------------------");

    tokio::select! {
        result = run_proxy_server() => {
            if let Err(e) = result {
                println!("Proxy server error: {}", e);
            }
        }
    }
    Ok(())
}