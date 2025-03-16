mod countries;

use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use log::{debug, error, info};
use constants::server_addresses::server_addresses::WORLDLIST_ADDR;
use engine::io::connection::{try_write_packet, Connection};
use engine::io::packet::Packet;
use countries::COUNTRY_MAP;

fn write_country_info(response: &mut Packet, country: &str) {
    let code = COUNTRY_MAP.get(country).expect(&format!("{} should be in the map", country));
    response.psmart(*code);
    response.pjstr2(country);
}

async fn handle_worldlist_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let addr = stream.peer_addr()?;
    debug!("New connection from: {}", addr);

    let mut connection = Connection::new(stream);

    loop {
        match connection.read_packet().await {
            Ok(0) => {
                debug!("Connection closed by client: {}", addr);
                break;
            }
            Ok(_n) => {
                let checksum = connection.inbound.g4();

                let mut response = Packet::from(vec![]);
                response.p1(1);

                if checksum != 2 {
                    response.p1(1); // Update
                    response.psmart(1); // Active world list

                    // World Block
                    write_country_info(&mut response, "Sweden");

                    response.psmart(1); // Offset
                    response.psmart(1); // Array size
                    response.psmart(1); // Active World count

                    // Sweden world
                    response.psmart(0);
                    response.p1(0);
                    response.p4(0);
                    response.pjstr2("");
                    response.pjstr2("localhost");

                    // Default value
                    response.p4(1);
                } else {
                    response.p1(0);
                }

                response.psmart(0);
                response.p2(40);
                response.psmart(1);
                response.p2(20);

                connection.outbound.p1(0);

                connection.outbound.p2(response.data.len() as i32);
                connection.outbound.pbytes(&response.data, 0, response.data.len());

                try_write_packet(&mut connection).await
            }
            Err(e) => {
                error!("Error reading from client: {}", e);
                break;
            }
        }
    }
    Ok(())
}

async fn run_worldlist_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(WORLDLIST_ADDR).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_worldlist_client(stream).await {
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

    info!("Starting Worldlist System");
    info!("---------------------------------------------");
    info!("Starting Worldlist server: {}", WORLDLIST_ADDR);
    info!("---------------------------------------------");

    tokio::select! {
        result = run_worldlist_server() => {
            if let Err(e) = result {
                error!("JS5 server error: {}", e);
            }
        }
    }

    Ok(())
}