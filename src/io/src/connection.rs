use std::io::repeat;
use std::sync::Arc;
use std::time::Duration;
use std::net::SocketAddr;
use tokio::net::TcpStream;

use log::{debug, error, info};
use rand::Rng;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use tokio::sync::Mutex;
use constants::js5_out::js5_out;
use constants::title_protocol::title_protocol;
use crate::packet::Packet;
use crate::client_state::ClientState;
use crate::js5_request_decoder::Js5RequestDecoder;


pub struct Connection {
    pub socket: TcpStream,
    pub state: ClientState,
    pub input: Packet,
    pub output: Packet,
    pub active: bool,
    pub peer_addr: SocketAddr,
}

impl Connection {
    pub async fn handle_connection(&mut self) {
        let mut buf = vec![0; 1024];

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
                        ClientState::Login => {
                            self.handle_login().await;
                        }
                        ClientState::Login_Secondary => {
                            self.handle_login_secondary().await;
                        }
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
                    // Timeout occurred
                    info!("No input received from {:?} within 5000 ms.", self.peer_addr);
                    let _ = self.socket.shutdown().await;
                    break;
                }
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub async fn handle_data_flush(&mut self) {
        let output_data = self.output.data.clone();

        info!("Flushing data: {:?} to: {:?}", output_data, self.peer_addr);
        match self.socket.write_all(&output_data).await {
            Ok(_) => {
                if let Err(e) = self.socket.flush().await {
                    error!("Failed to flush: {} to: {:?}", e, self.peer_addr);
                    self.state = ClientState::Closed;
                }
                self.output = Packet::from(vec![]);
            },
            Err(e) => {
                error!("Failed to write: {} to: {:?}", e, self.peer_addr);
                self.state = ClientState::Closed;
            }
        }
    }

    async fn handle_new_connection(&mut self) {
        info!("New connection from: {:?}", self.peer_addr);

        if self.input.remaining() <= 0 {
            // No input to process
            return;
        }

        let opcode = self.input.g1();
        info!("Received opcode is {}", opcode);

        match opcode {
            title_protocol::WORLD_HANDSHAKE => {
                self.handle_login().await;
            }

            title_protocol::JS5OPEN => {
                let client_version = self.input.g4();
                info!("Client version is {}", client_version);

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
                self.state = ClientState::Closed;
                debug!("Unhandled opcode received: {}", opcode);
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

    async fn handle_login(&mut self) {
        debug!("Login connection from: {:?}", self.peer_addr);
        let playerHash = self.input.g1();

        self.output.p1(0);

        let session_key = 56468456468454; // TODO - actual rng implementation
        self.output.p8(session_key.clone());
        self.state = ClientState::Login_Secondary;
        self.handle_data_flush().await;
    }

    async fn handle_login_secondary(&mut self) {
        debug!("Login secondary connection from: {:?}", self.peer_addr);
        let opcode = self.input.g1();
        debug!("Received opcode is {}", opcode);

        if opcode != 16 && opcode != 18 {
            self.output.p1(22); // TODO - Const this
            self.handle_data_flush().await;
            self.state = ClientState::Closed;
            return
        }

        let length = self.input.g2();
        let client_version = self.input.g4();

        if client_version != 530 {
            self.output.p1(6);
            self.handle_data_flush().await;
            self.state = ClientState::Closed;
            return
        }

        let byte1 = self.input.g1s();
        let adverts_suppressed = self.input.g1s();
        let byte2 = self.input.g1s();
        let window_mode = self.input.g1s();
        let canvas_width = self.input.g2();
        let canvas_height = self.input.g2();
        let anti_aliasing = self.input.g1s();
        let uid = self.input.gbytes(24);
        let client_settings = self.input.gjstr(0);
        let affiliate = self.input.g4();
        let preferences = self.input.g4();

        let client_verify_id = self.input.g2();

        let mut checksums = Vec::new();
        for _ in 0..28 {
            checksums.push(self.input.g4());
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
        self.output.pbytes(&temp, response.data.len());

        self.handle_data_flush().await;
    }
}