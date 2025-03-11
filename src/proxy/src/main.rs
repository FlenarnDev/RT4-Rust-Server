use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use log::debug;
use socket2::{Socket, Domain, Type};

// Connection status enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionStatus {
    Active,
    Idle,
    Closed,
}

// Connection info structure
struct ConnectionInfo {
    stream: TcpStream,
    status: ConnectionStatus,
    last_activity: Instant,
    destination_server: String,
}

// Proxy server structure
struct ProxyServer {
    connections: HashMap<String, ConnectionInfo>,
}

impl ProxyServer {
    fn new() -> Self {
        ProxyServer {
            connections: HashMap::new(),
        }
    }

    // Add or update a connection
    fn update_connection(&mut self, key: String, stream: TcpStream, destination: String) {
        let info = ConnectionInfo {
            stream,
            status: ConnectionStatus::Active,
            last_activity: Instant::now(),
            destination_server: destination,
        };
        self.connections.insert(key, info);
    }

    // Update connection status
    fn set_connection_status(&mut self, key: &str, status: ConnectionStatus) {
        if let Some(conn) = self.connections.get_mut(key) {
            conn.status = status;
            if status == ConnectionStatus::Active {
                conn.last_activity = Instant::now();
            }
        }
    }

    // Remove a connection
    fn remove_connection(&mut self, key: &str) {
        self.connections.remove(key);
    }

    // Get connection status
    fn get_connection_status(&self, key: &str) -> Option<ConnectionStatus> {
        self.connections.get(key).map(|conn| conn.status)
    }

    // Check for timed out connections and clean them up
    fn cleanup_connections(&mut self) {
        let now = Instant::now();
        let timeout = Duration::from_millis(500); // 500ms timeout

        let keys_to_remove: Vec<String> = self.connections.iter()
            .filter(|(_, conn)| {
                conn.status == ConnectionStatus::Idle &&
                    now.duration_since(conn.last_activity) > timeout
            })
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            println!("Connection timed out: {}", key);
            if let Some(mut conn) = self.connections.remove(&key) {
                // Send FIN packet or other cleanup
                let _ = conn.stream.shutdown(std::net::Shutdown::Both);
            }
        }
    }

    // Get statistics about connections
    fn get_stats(&self) -> (usize, usize, usize) {
        let active = self.connections.values()
            .filter(|c| c.status == ConnectionStatus::Active)
            .count();
        let idle = self.connections.values()
            .filter(|c| c.status == ConnectionStatus::Idle)
            .count();
        let closed = self.connections.values()
            .filter(|c| c.status == ConnectionStatus::Closed)
            .count();

        (active, idle, closed)
    }
}

fn main() -> std::io::Result<()> {
    // Add socket2 crate to Cargo.toml
    // [dependencies]
    // socket2 = "0.5.3"

    // Create the TCP listener
    let listener = TcpListener::bind("127.0.0.1:40000")?;
    println!("Server listening on port 8080");

    // Create the shared proxy state
    let proxy = Arc::new(Mutex::new(ProxyServer::new()));

    // Start the cleanup thread
    let cleanup_proxy = Arc::clone(&proxy);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100)); // Check every 100ms
            let mut proxy = cleanup_proxy.lock().unwrap();
            proxy.cleanup_connections();

            // Log current connection stats
            let (active, idle, _) = proxy.get_stats();
            println!("Connections - Active: {}, Idle: {}", active, idle);
        }
    });

    // Accept connections and process them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let proxy_clone = Arc::clone(&proxy);

                // Set SO_KEEPALIVE on the socket
                if let Ok(stream_clone) = stream.try_clone() {
                    // Convert TcpStream to socket2::Socket for more options
                    let socket = Socket::from(stream_clone);
                    socket.set_keepalive(true)?;

                    // On some platforms, you can configure keep-alive parameters
                    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
                    {
                        // Set keep-alive time (time before sending probes)
                        socket.set_tcp_keepalive(&socket2::TcpKeepalive::new().with_time(Duration::from_millis(500)))?;
                    }
                }

                // Get peer address for connection tracking
                let peer_addr = match stream.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "unknown".to_string(),
                };

                // Generate a unique connection ID
                let conn_id = format!("conn_{}", peer_addr);

                thread::spawn(move || {
                    handle_client(stream, conn_id, proxy_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

// Client handler function
fn handle_client(mut stream: TcpStream, conn_id: String, proxy: Arc<Mutex<ProxyServer>>) {
    // Get destination server (simplified - in a real proxy you'd parse the request)
    let destination = "example.com:80".to_string();

    // Register the connection
    {
        let mut proxy_guard = proxy.lock().unwrap();
        proxy_guard.update_connection(conn_id.clone(), stream.try_clone().unwrap(), destination.clone());
    }

    println!("New connection: {}", conn_id);

    // Buffer for reading data
    let mut buffer = [0; 1024];

    // Main connection loop
    loop {
        // Set read timeout to detect inactive connections
        let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));

        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by client
                println!("Connection closed by client: {}", conn_id);
                let mut proxy_guard = proxy.lock().unwrap();
                proxy_guard.set_connection_status(&conn_id, ConnectionStatus::Closed);
                proxy_guard.remove_connection(&conn_id);
                break;
            }
            Ok(bytes_read) => {
                // Got data, connection is active
                let mut proxy_guard = proxy.lock().unwrap();
                proxy_guard.set_connection_status(&conn_id, ConnectionStatus::Active);
                drop(proxy_guard); // Release the lock
                debug!("Read {} bytes from stream: {}", bytes_read, conn_id);

                let mut zero_vec = vec![0];
                zero_vec.push(0);
                
                // Echo the data back (in a real proxy, you'd forward to destination)
                if let Err(e) = stream.write(&*zero_vec) {
                    eprintln!("Error writing to stream: {}", e);
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock ||
                e.kind() == std::io::ErrorKind::TimedOut => {
                // No data available, connection is idle
                let mut proxy_guard = proxy.lock().unwrap();
                proxy_guard.set_connection_status(&conn_id, ConnectionStatus::Idle);
            }
            Err(e) => {
                // Other errors
                eprintln!("Error reading from stream: {}", e);
                let mut proxy_guard = proxy.lock().unwrap();
                proxy_guard.set_connection_status(&conn_id, ConnectionStatus::Closed);
                proxy_guard.remove_connection(&conn_id);
                break;
            }
        }
        debug!("buffer: {:?}", buffer);
    }
}