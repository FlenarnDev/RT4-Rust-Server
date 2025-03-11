use std::cmp::min;
use std::error::Error;
use std::thread::sleep;
use log::debug;
use rs2cache::store::ARCHIVESET;
use cache::file_handler::{ensure_initialized, CACHE, MASTER_INDEX_VEC};
use cache::version_trailer::VersionTrailer;
use io::connection::Connection;
use io::packet::Packet;

const BLOCK_SIZE: usize = 512;
pub const BLOCK_HEADER_SIZE: usize = 1 + 2 + 1;
pub const BLOCK_DELIMITER_SIZE: usize = 1;
pub const BYTES_BEFORE_BLOCK: usize = BLOCK_SIZE - BLOCK_HEADER_SIZE;
pub const BYTES_AFTER_BLOCK: usize = BLOCK_SIZE - BLOCK_DELIMITER_SIZE;

#[derive(Debug)]
pub (crate) enum Js5Request {
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
    Invalid
}

impl Js5Request {
    pub fn fulfill_request(connection: &mut Connection, request: &Js5Request) -> Result<(), Box<dyn Error>> {
        debug!("Fulfilling request: {:?}", request);

        // Ensure the cache is initialized in this thread before proceeding
        ensure_initialized()?;

        if let Js5Request::Group { urgent, archive, group } = request {
            if *archive == ARCHIVESET && *group == ARCHIVESET as u16 {
                // Handle master index request
                let mut success = false;

                MASTER_INDEX_VEC.with(|master_vec_ref| {
                    if let Some(master_index_vec) = &*master_vec_ref.borrow() {
                        let master_index_length = master_index_vec.len();
                        connection.outbound = Packet::new(8 + master_index_length);
                        connection.outbound.p1(ARCHIVESET as i32);
                        connection.outbound.p2(ARCHIVESET as i32);
                        connection.outbound.p1(0);
                        connection.outbound.p4(master_index_length as i32);

                        connection.outbound.pbytes(&master_index_vec, 0, master_index_length);
                        success = true;
                    }
                });

                if !success {
                    return Err("Master index not initialized".into());
                }
            } else {
                // Handle regular file request
                let data = CACHE.with(|cache_ref| -> Result<Vec<u8>, Box<dyn Error>> {
                    let cache_ref = cache_ref.borrow();
                    if let Some(cache) = &*cache_ref {
                        Ok(cache.store.read(*archive, *group as u32)?)
                    } else {
                        Err("Cache not initialized".into())
                    }
                })?;

                let length = 2 + data.len() + (512 + data.len()) / 511;
                //debug!("Length of data packet: {}", data.len());
                //debug!("Data packet1: {:?}", data);
                //debug!("Estimated length of final packet: {}", length);

                let mut data_packet = Packet::from(data);
                connection.outbound = Packet::new(length);

                connection.outbound.p1(*archive as i32);
                connection.outbound.p2(*group as i32);

                let mut compression = data_packet.data[0];
                if *urgent {
                    compression |= 0x80;
                }
                connection.outbound.p1(compression as i32);

                let data_to_write = data_packet.gbytes(min(data_packet.remaining() as usize, 508));
                connection.outbound.pbytes(&*data_to_write, 0, data_to_write.len());

                while data_packet.remaining() > 0 {
                    connection.outbound.p1(255);
                    let data_to_add = data_packet.gbytes(min(data_packet.remaining() as usize, BYTES_AFTER_BLOCK));
                    connection.outbound.pbytes(&*data_to_add, 0, data_to_add.len());
                }
            }

            Ok(())
        } else {
            Err("Invalid JS5 request".into())
        }
    }
}