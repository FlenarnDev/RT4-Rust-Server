use std::sync::Arc;
use std::error::Error;

use tokio::net::TcpListener;
use tokio::spawn;
use tokio::sync::Mutex;

use log::{debug, error, info};

use io::client_state::ClientState;
use io::connection::Connection;
use io::packet::Packet;

async fn async_main() -> Result<(), Box<dyn Error>> {
    let js5_listener = TcpListener::bind("127.0.0.1:40000").await?;
    info!("Listening on: {}", "127.0.0.1:40000");

    cache::file_handler::init();

    loop {
        match js5_listener.accept().await {
            Ok((socket, _)) => {
                let peer_addr = socket.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());

                // Wrap the connection in Arc<Mutex> to safely share it between tasks
                let connection = Arc::new(Mutex::new(Connection {
                    socket,
                    state: ClientState::New,
                    input: Packet::from(vec![]),
                    output: Packet::from(vec![]),
                    active: true,
                    peer_addr,
                }));

                // Use Arc<Mutex> inside the async block
                spawn(async move {
                    let mut conn = connection.lock().await;
                    conn.handle_connection().await;
                });
            },
            Err(e) => error!("Failed to accept connection: {}", e),
        }
    }
}

fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build() {
        Ok(runtime) => {
            if let Err(e) = runtime.block_on(async_main()) {
                error!("Error in async runtime: {}", e);
            }
        },
        Err(e) => {
            error!("Failed to build runtime: {}", e);
        }
    }
}
