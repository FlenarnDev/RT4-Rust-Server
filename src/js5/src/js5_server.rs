use tokio::net::TcpListener;
use log::{debug, error};
use io::client_state::ClientState;
use io::connection::Connection;
use io::packet::Packet;
use crate::js5_request_decoder::Js5RequestDecoder;

async fn handle_js5_connection(mut connection: Connection) -> std::io::Result<()> {
    debug!("Handling JS5 connection from {}", connection.peer_addr);

    match Js5RequestDecoder::process(&mut connection).await {
        Ok(_) => debug!("Successfully processed JS5 request."),
        Err(e) => {
            error!("Error processing JS5 request. {}", e);
        }
    }
    Ok(())
}

pub async fn js5_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:43595").await?;
    println!("JS5 server listening on 127.0.0.1:43595");

    while let Ok((socket, peer_addr)) = listener.accept().await {
        let conn = Connection {
            socket,
            state: ClientState::CONNECTED,
            input: Packet::from(Vec::new()),
            output: Packet::from(Vec::new()),
            active: true,
            peer_addr,
        };

        tokio::spawn(async move {
            if let Err(e) = handle_js5_connection(conn).await {
                eprintln!("Error handling JS5 connection: {:?}", e);
            }
        });
    }

    Ok(())
}
