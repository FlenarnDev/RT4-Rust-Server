use std::cmp::min;
use std::error::Error;
use log::debug;
use rs2cache::store::ARCHIVESET;
use constants::js5_service::js5_service::{HEADER, OFFSET, SEPARATOR, SPLIT};
use io::connection::Connection;
use cache::file_handler::{CACHE, MASTER_INDEX_VEC};
use cache::version_trailer::VersionTrailer;
use io::packet::Packet;

#[derive(Debug, Clone, PartialEq)]
pub enum Js5Request {
    Group {
        prefetch: bool,
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
    fn get_int(b1: u8, b2: u8, b3: u8, b4: u8) -> i32 {
        ((b1 as i32) << 24)
            | ((b2 as i32 & 0xff) << 16)
            | ((b3 as i32 & 0xff) << 8)
            | (b4 as i32 & 0xff)
    }
    
    pub(crate) async fn fulfill(connection: &mut Connection, js5request: &Js5Request) -> Result<(), Box<dyn Error>> {
        debug!("Fulfilling request: {:?}", js5request);
        if let Js5Request::Group { prefetch, archive, group } = js5request {
            

            let cache = CACHE.get()
                .ok_or("Cache not initialized")?;

            if *archive == ARCHIVESET && *group == ARCHIVESET as u16 {
                let master_index_length = MASTER_INDEX_VEC.get().unwrap().len();
                connection.output = Packet::new(8 + master_index_length);
                connection.output.p1(ARCHIVESET as i32);
                connection.output.p2(ARCHIVESET as i32);
                connection.output.p1(0);
                connection.output.p4(master_index_length as i32);

                connection.output.pbytes(&MASTER_INDEX_VEC.get().unwrap(), 0, master_index_length);
            } else {
                debug!("Fulfilling request: Archive: {}, Group: {}", *archive, *group);
                connection.output.p1(*archive as i32);
                connection.output.p2(*group as i32);

                let mut request_data = cache.store.read(*archive, *group as u32).expect("Failed to read archive & group.");
                debug!("Read {} bytes from archive & group.", request_data.len());
                if *archive != ARCHIVESET {
                    VersionTrailer::strip(&mut request_data);
                }
                
                let mut compression: u32 = request_data[0] as u32;
                if *prefetch {
                    compression = compression | 0x80;
                }
                connection.output.p1(compression as i32);
                
                let size = Self::get_int(request_data[1], request_data[2], request_data[3], request_data[4]) as usize;
                let mut length = min(size, (SPLIT - HEADER) as usize);
                
                connection.output.pbytes(&request_data, 0, min(request_data.len(), 508));
                let mut written = length;
                

                while written < size {
                    debug!("Writing split");
                    connection.output.p1(SEPARATOR);
                    length = min(size - written, (SPLIT - 1) as usize);
                    connection.output.pbytes(&request_data, 0, min(request_data.len(), 511));
                    written += length;
                }
            }
            connection.handle_data_flush().await;
            Ok(())
        } else {
            Err("Expected Group request".into())
        }
    }
}