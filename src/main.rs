use cache::file_handler;
use tokio::net::TcpListener;
use tokio::io::Result;

use io::packet::Packet;

use log::{debug, info};
use tokio::runtime::Runtime;
use io::client_state::ConnectionState;
use io::connection::{handle_connection, Connection};
use js5::main::js5_server;
use worldlist::worldlist_server::worldlist_server;

fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    file_handler::init();

    let rt = Runtime::new()?;

    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:40000").await?;
    info!("Main server listening on 127.0.0.1:40000");

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
            state: ConnectionState::CONNECTED,
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