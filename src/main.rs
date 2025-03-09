use cache::file_handler;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result, AsyncReadExt, AsyncWriteExt, copy};
use std::net::SocketAddr;
use std::pin::Pin;
use std::future::Future;

use io::packet::Packet;

use log::debug;
use tokio::runtime::Runtime;
use io::client_state::ClientState;
use io::connection::{handle_connection, Connection};
use js5::js5_server::js5_server;
use worldlist::worldlist_server::worldlist_server;

fn main() -> Result<()> {
    // Synchronous code to run before starting the async runtime
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    file_handler::init();

    // Create the async runtime
    let rt = Runtime::new()?;

    // Run the async main function within the runtime
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:40000").await?;
    println!("Main server listening on 127.0.0.1:40000");

    tokio::spawn(async {
        if let Err(e) = js5_server().await {
            eprintln!("Error running JS5 server: {:?}", e);
        }
    });
    
    tokio::spawn(async {
        if let Err(e) = worldlist_server().await {
            eprintln!("Error running worldlist server: {:?}", e);
        }
    });

    while let Ok((socket, peer_addr)) = listener.accept().await {
        let conn = Connection {
            socket,
            state: ClientState::CONNECTED,
            input: Packet::from(Vec::new()),
            output: Packet::from(Vec::new()),
            active: true,
            peer_addr,
        };

        debug!("Accepted connection from {}", conn.peer_addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(conn).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }

    Ok(())
}