use crate::js5_request::Js5Request;
use std::error::Error;
use log::{debug, error};

use constants::js5_in::js5_in;
use io::connection::Connection;

pub struct Js5RequestDecoder;

impl Js5RequestDecoder {
    async fn decode(connection: &mut Connection) -> Result<Js5Request, Box<dyn Error>> {
        let mut request = Js5Request::Invalid;
        
        let opcode = connection.input.g1();
        debug!("JS5 opcode is {}", opcode);
        request = match opcode {
            js5_in::REQUEST | js5_in::PRIORITY_REQUEST => {
                let urgent = opcode == js5_in::PRIORITY_REQUEST;
                let archive = connection.input.g1();
                let group = connection.input.g2();

                Js5Request::Group {
                    prefetch: urgent,
                    archive,
                    group
                }
            },
            js5_in::REKEY => {
                let key = connection.input.g1();
                let _unknown = connection.input.g2();
                Js5Request::Rekey { key }
            },
            js5_in::LOGGED_IN => {
                connection.input.g3();
                Js5Request::LoggedIn
            },
            js5_in::LOGGED_OUT => {
                connection.input.g3();
                Js5Request::LoggedOut
            },
            js5_in::CONNECTED => {
                connection.input.g3();
                Js5Request::Connected
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
    
    pub async fn process(connection: &mut Connection) -> Result<(), Box<dyn Error>> {
        while connection.input.remaining() > 0 {
            let request = Self::decode(connection).await?;
            match &request {
                Js5Request::Group{..} => {
                    Js5Request::fulfill(connection, &request).await?;
                },
                Js5Request::Rekey { key } => {
                    debug!("JS5 Rekey request with key: {}", key);
                },
                Js5Request::LoggedIn => {
                    debug!("JS5 LoggedIn request");
                },
                Js5Request::LoggedOut => {
                    debug!("JS5 LoggedOut request");
                },
                Js5Request::Connected => {
                    debug!("JS5 Connected request");
                },
                Js5Request::Disconnect => {
                    debug!("JS5 Disconnect request");
                },
                Js5Request::Invalid => {
                    error!("Invalid JS5 request");
                },
            }
        }
        Ok(())
    }
}