use crate::js5_request::Js5Request;
use std::error::Error;
use log::{debug, error};

use constants::js5_in::js5_in;
use crate::client_state::ClientState;
use crate::connection::Connection;

pub struct Js5RequestDecoder;

impl Js5RequestDecoder {
    async fn decode(connection: &mut Connection) -> Result<Js5Request, Box<dyn Error>> {
        let mut request = Js5Request::Invalid;

        while connection.input.remaining() > 0 {
            let opcode = connection.input.g1();
            debug!("JS5 opcode is {}", opcode);

            request = match opcode {
                js5_in::REQUEST | js5_in::PRIORITY_REQUEST => {
                    let urgent = opcode == js5_in::PRIORITY_REQUEST;
                    let archive = connection.input.g1();
                    let group = connection.input.g2();

                    Js5Request::Group {
                        urgent,
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
        }

        Ok(request)
    }

    pub(crate) async fn process(connection: &mut Connection) -> Result<(), Box<dyn Error>> {
        let request = Self::decode(connection).await?;

        match &request {
            Js5Request::Group{..} => {
                Js5Request::fulfill(connection, &request).await?;
            },
            Js5Request::Rekey { key } => {
                debug!("JS5 Rekey request with key: {}", key);
                // Handle rekey
            },
            Js5Request::LoggedIn => {
                debug!("JS5 LoggedIn request");
                // Handle logged in
            },
            Js5Request::LoggedOut => {
                debug!("JS5 LoggedOut request");
                // Handle logged out
            },
            Js5Request::Connected => {
                debug!("JS5 Connected request");
                // Handle connected
            },
            Js5Request::Disconnect => {
                debug!("JS5 Disconnect request");
                // Handle disconnect
            },
            Js5Request::Invalid => {
                error!("Invalid JS5 request");
                // Handle invalid
            },
        }

        Ok(())
    }
}