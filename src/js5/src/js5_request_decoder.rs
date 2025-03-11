use constants::js5_in::js5_in;
use io::connection::Connection;
use log::{debug, error};
use crate::js5_request;
use crate::js5_request::Js5Request;

///All upstream packets are exactly 4 bytes long, including the opcode.
/// Unused payload bytes are set to zero.
pub(crate) async fn process(connection: &mut Connection) {
    //while connection.inbound().remaining() > 0 {
        let opcode = connection.inbound().g1();
        let request = match opcode {
            js5_in::PREFETCH | js5_in::URGENT => {
                Js5Request::Group {
                    urgent: opcode == js5_in::URGENT,
                    archive: connection.inbound.g1(),
                    group: connection.inbound.g2()
                }
            }

            js5_in::REKEY => {
                let key = connection.inbound.g1();
                if connection.inbound.g2() != 0 {
                    Js5Request::Invalid
                } else {
                    Js5Request::Rekey { key }
                }
            }
            js5_in::LOGGED_IN => {
                if connection.inbound.g3() != 0 {
                    Js5Request::Invalid
                } else {
                    Js5Request::LoggedIn
                }
            }
            js5_in::LOGGED_OUT => {
                connection.inbound().g3();
                Js5Request::LoggedOut
            }
            js5_in::CONNECTED => {
                // Value is always '3'.
                if connection.inbound.g3() != 3 {
                    Js5Request::Invalid
                } else {
                    Js5Request::Connected
                }
            }
            js5_in::DISCONNECT => {
                connection.inbound().g3();
                Js5Request::Disconnect
            }
            _ => {
                debug!("Invalid opcode: {}", opcode);
                connection.inbound().g3();
                Js5Request::Invalid
            }
        };

        debug!("Received JS5 request: {:?}", request);

        match request {
            Js5Request::Group { .. } => {
                Js5Request::fulfill_request(connection, &request).unwrap();
            }

            Js5Request::Invalid => {
            // TODO - terminate early.
            }
            _ => {
                // Currently nothing.
            }
        }

        // Send response if outbound isn't empty
        if !connection.outbound.is_empty() {
            match connection.write_packet().await {
                Ok(bytes_written) => {
                    debug!("Sent response packet: {} bytes", bytes_written);
                },
                Err(e) => {
                    error!("Error writing to client: {}", e);
                    
                }
            }
        }
    //}
}