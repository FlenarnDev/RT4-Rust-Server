use std::error::Error;
use log::{debug, error};
use tokio::io::AsyncReadExt;
use constants::js5_in::js5_in;
use io::client_state::ClientState;
use io::connection::Connection;
use io::packet::Packet;
use crate::js5_request::Js5Request;

pub struct Js5RequestDecoder;

impl Js5RequestDecoder {
    /// `All upstream packets are exactly 4 bytes long, including the opcode. Unused payload bytes are set to zero.`
    /// @Graham
    /// Make use of that information for packet verifications where suitable.
    fn decode(connection: &mut Connection) -> Result<Js5Request, Box<dyn Error>> {
        let opcode = connection.input.g1();
        let request = match opcode {
            js5_in::PREFETCH | js5_in::URGENT => {
                let prefetch = opcode == js5_in::URGENT;
                let archive = connection.input.g1();
                let group = connection.input.g2();
                
                Js5Request::Group {
                    prefetch,
                    archive,
                    group
                }
            }
            js5_in::REKEY => {
                let key = connection.input.g1();
                if connection.input.g2() != 0 {
                    Js5Request::Invalid
                } else {
                    Js5Request::Rekey { key }
                }
            },
            js5_in::LOGGED_IN => {
                if connection.input.g3() != 0 {
                    Js5Request::Invalid
                } else {
                    Js5Request::LoggedIn
                }
            },
            js5_in::LOGGED_OUT => {
                connection.input.g3();
                Js5Request::LoggedOut
            },
            js5_in::CONNECTED => {
                if connection.input.g3() != 3 {
                    Js5Request::Invalid
                } else {
                    Js5Request::Connected
                }
            }
            js5_in::DISCONNECT => {
                connection.input.g3();
                Js5Request::Disconnect
            },
            _ => {
                connection.input.g3();
                Js5Request::Invalid
            }
        };
        Ok(request)
    }
    
    pub(crate) async fn process(connection: &mut Connection) -> Result<Js5Request, Box<dyn Error>> {
        let mut request = Js5Request::Invalid;

        while connection.active {
            let mut buffer = [0; 1024];
            let n = connection.socket.read(&mut buffer).await?;
            connection.input = Packet::from(buffer[..n].to_vec());

            while connection.input.remaining() > 0 {
                request = Js5RequestDecoder::decode(connection)?;
                match &request {
                    Js5Request::Group{..} => {
                        debug!("JS5 Group request with: {:?}", request);
                        Js5Request::fulfill_request(connection, &request).await.expect("Failed to fulfill JS5 request");
                    },
                    Js5Request::Rekey { key } => {
                        debug!("JS5 Rekey request with key: {}", key);
                    },
                    Js5Request::LoggedIn => {
                        debug!("JS5 LoggedIn request");
                    },
                    Js5Request::LoggedOut => {
                        debug!("JS5 LoggedOut request");
                        connection.state = ClientState::CLOSED;
                        break;
                    },
                    Js5Request::Connected => {
                        debug!("JS5 Connected request");
                    },
                    Js5Request::Disconnect => {
                        debug!("JS5 Disconnect request");
                        connection.state = ClientState::CLOSED;
                        break;
                    },
                    Js5Request::Invalid => {
                        error!("Invalid JS5 request");
                        connection.state = ClientState::CLOSED;
                        break;
                    },
                }
            }
        }
        
        Ok(request)
    }
}