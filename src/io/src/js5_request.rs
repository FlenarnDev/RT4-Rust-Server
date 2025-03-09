use std::cmp::min;
use std::error::Error;
use log::debug;
use rs2cache::store::ARCHIVESET;
use constants::js5_service::js5_service::{HEADER, SPLIT};
use crate::connection::Connection;
use cache::file_handler::{CACHE, MASTER_INDEX_VEC};
use cache::version_trailer::VersionTrailer;
use crate::packet::Packet;

#[derive(Debug, Clone, PartialEq)]
pub enum Js5Request {
    Group {
        urgent: bool,
        archive: u8,
        group: u16,
    },
    LoggedIn,
    LoggedOut,
    Rekey {
        key: u8,
    },
    Connected,
    Disconnect,
    Invalid,
}

impl Js5Request {
    pub(crate) async fn fulfill(connection: &mut Connection, js5request: &Js5Request) -> Result<(), Box<dyn Error>> {
        if let Js5Request::Group { urgent, archive, group } = js5request {
            let mut request_data = vec![];

            let cache = CACHE.get()
                .ok_or("Cache not initialized")?;

            if *archive == ARCHIVESET && *group == ARCHIVESET as u16 {
                debug!("Serving main index (255,255)");
                request_data = MASTER_INDEX_VEC.get().unwrap().clone();
                connection.output = Packet::new(8 + request_data.len());
                connection.output.p1(ARCHIVESET as i32);
                connection.output.p2(ARCHIVESET as i32);
                connection.output.p1(0);
                connection.output.p4(request_data.len() as i32);

                connection.output.pbytes(&request_data, request_data.len());
            } else {
                request_data = cache.store.read(*archive, *group as u32).expect("Failed to read archive & group.");
                if *archive != ARCHIVESET {
                    VersionTrailer::strip(&mut request_data);
                }
                let mut request_packet = Packet::from(request_data.clone());

                let compression = request_packet.g1();
                let size: i32 = request_packet.g4();
                request_packet = Packet::from(vec![]);

                request_packet.p1(*archive as i32);
                request_packet.p2(*group as i32);
                request_packet.p1(if *urgent { (compression | 0x80) as i32 } else { compression as i32 });

                // Serving starts here
                let length = min(size, SPLIT - HEADER);
                debug!("Serving archive: {}, group: {}, length: {}", *archive, *group, length);
                debug!("Compression: {}", compression);
                debug!("Size: {}", size);
                debug!("Urgent: {}", *urgent);
                request_packet.pbytes(&request_data, length as usize);

                connection.output = request_packet;
            }

            connection.handle_data_flush().await;
            Ok(())
        } else {
            Err("Expected Group request".into())
        }
    }
}