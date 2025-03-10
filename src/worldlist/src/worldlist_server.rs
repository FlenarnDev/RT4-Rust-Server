use tokio::net::TcpListener;
use log::{debug, error};
use tokio::io::AsyncReadExt;
use io::client_state::ClientState;
use io::connection::{write_and_clear_output, Connection};
use io::packet::Packet;
use crate::countries::COUNTRY_MAP;

fn write_country_info(response: &mut Packet, country: &str) {
    let code = COUNTRY_MAP.get(country).expect(&format!("{} should be in the map", country));
    response.psmart(*code);
    response.pjstr2(country);
}

async fn process(connection: &mut Connection) -> std::io::Result<()> {
    let mut buffer = [0; 4];
    let n = connection.socket.read(&mut buffer).await?;
    connection.input = Packet::from(buffer[..n].to_vec());
    let checksum = connection.input.g4();

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
    
    connection.output.p1(0);
    
    connection.output.p2(response.data.len() as i32);
    connection.output.pbytes(&response.data, 0, response.data.len());

    write_and_clear_output(connection).await?;
    connection.state = ClientState::CLOSED;
    Ok(())
}

async fn handle_worldlist_connection(mut connection: Connection) -> std::io::Result<()> {
    debug!("Handling worldlist connection from {}", connection.peer_addr);
    match process(&mut connection).await { 
        Ok(_) => {
            debug!("Worldlist connection processed successfully");
        },
        Err(e) => {
            error!("Error processing worldlist connection: {:?}", e);
        }
    }
    Ok(())
}

pub async fn worldlist_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:43596").await?;
    println!("Worldlist server listening on 127.0.0.1:43596");
    
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
            if let Err(e) = handle_worldlist_connection(conn).await {
                error!("Error handling connection: {:?}", e);
            }
        });
    }
    Ok(())
}