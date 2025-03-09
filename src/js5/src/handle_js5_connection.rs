use std::time::Duration;
use async_trait::async_trait;
use log::{debug, error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use constants::js5_out::js5_out;
use constants::title_protocol::title_protocol;
use io::client_state::ClientState;
use io::connection::Connection;
use io::packet::Packet;
use crate::js5_request_decoder::Js5RequestDecoder;

#[async_trait]
pub trait ConnectionJs5 {
    fn new() -> Self;
    async fn handle_js5_connection(&mut self);
    async fn handle_new_connection(&mut self);
    async fn handle_js5(&mut self);
    async fn handle_worldlist_fetch(&mut self);
}

#[async_trait]
impl ConnectionJs5 for Connection {
    fn new() -> Self {
        unimplemented!();
    }
    
    async fn handle_js5_connection(&mut self) {
        let mut buf: Vec<u8> = vec![0; 1024];
        
        while self.active {
            debug!("Handling: {:?}", self.peer_addr);
            match timeout(Duration::from_secs(5), self.socket.read(&mut buf)).await {
                Ok(Ok(0)) => {
                    // Connection closed
                    self.state = ClientState::Closed;
                    break;
                },
                Ok(Ok(n)) => {
                    // Process input
                    self.input = Packet::from(buf[..n].to_vec());

                    match self.state {
                        ClientState::Closed => {
                            self.active = false;
                            let _ = self.socket.shutdown().await;
                        },
                        ClientState::New => {
                            self.handle_new_connection().await;
                        },
                        ClientState::Js5 => {
                            self.handle_js5().await;
                        },
                        _ => {
                            let _ = self.socket.shutdown().await;
                        }
                    }
                },
                Ok(Err(e)) => {
                    error!("Error reading from socket: {}", e);
                    self.active = false;
                    let _ = self.socket.shutdown().await;
                    break;
                },
                Err(_) => {
                    debug!("No input received from {:?} within 5000 ms.", self.peer_addr);
                    let _ = self.socket.shutdown().await;
                    break;
                }
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    
    async fn handle_new_connection(&mut self) {
        debug!("New connection from: {:?}", self.peer_addr);
        
        if self.input.remaining() <= 0 {
            // No input to process
            return;
        }
        
        let opcode = self.input.g1();
        debug!("Received opcode: {}", opcode);
        
        match opcode { 
            title_protocol::JS5OPEN => {
                let client_version = self.input.g4();
                debug!("Client version: {}", client_version);
                
                if client_version == 530 {
                    self.output.p1(js5_out::SUCCESS);
                    self.state = ClientState::Js5;
                } else {
                    self.output.p1(js5_out::OUT_OF_DATE);
                    self.state = ClientState::Closed;
                }
                self.handle_data_flush().await;
            },
            title_protocol::WORLDLIST_FETCH => {
                self.handle_worldlist_fetch().await;
                self.state = ClientState::Closed;
            },
            _ => {
                debug!("Unhandled opcode received: {}", opcode);
                self.state = ClientState::Closed;
            }
        }
    }
    
    async fn handle_js5(&mut self) {
        debug!("JS5 connection from: {:?}", self.peer_addr);
        
        match Js5RequestDecoder::process(self).await {
            Ok(_) => debug!("Successfully processed JS5 request."),
            Err(e) => {
                error!("Error processing JS5 request. {}", e);
            }
        }
    }
    
    async fn handle_worldlist_fetch(&mut self) {
        debug!("Worldlist fetch from: {:?}", self.peer_addr);
        self.output.p1(0); // Response code - TODO
        let checksum = self.input.g4();

        let mut response = Packet::from(Vec::new());
        response.p1(1); // Version

        if checksum != 2 {
            response.p1(1); // Update
            response.psmart(1); // Active world list

            // World Block
            response.psmart(191);
            response.pjstr2("Sweden");

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
        debug!("worldlist data: {:?}", response.data);

        // Predefined response data
        let temp = vec![
            1, 1, 1, 128, 191, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 40, 1, 0, 20
        ];

        self.output.p2(response.data.len() as i32);
        self.output.pbytes(&temp, 0, response.data.len());

        self.handle_data_flush().await;
    }
}