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

                // Make sure the data is properly "trimmed" to match Go's behavior
                // In Go: return data[0 : size+headerLength]
                let data_length = data.len();
                //debug!("Raw data length: {}", data_length);

                // Following Go's writeChunked logic more closely
                // Calculate total packet size including potential delimiters
                let mut total_size = 3; // Initial header size: index(1) + file(2)
                let mut remaining = data_length;
                let mut position = 3;

                // Calculate how many delimiters we'll need
                while remaining > 0 {
                    let block_len = min(remaining, 512 - (position % 512));
                    position += block_len;
                    remaining -= block_len;

                    if position % 512 == 0 && remaining > 0 {
                        total_size += 1; // Add one for the delimiter
                        position += 1;
                    }
                }

                total_size += data_length;
                //debug!("Calculated total size: {}", total_size);

                // Create new outbound packet
                connection.outbound = Packet::new(total_size);

                // Write header
                connection.outbound.p1(*archive as i32);
                connection.outbound.p2(*group as i32);

                // Reset counters for actual writing
                remaining = data_length;
                position = 3;
                let mut src_pos = 0;

                // Write data in chunks using Go's algorithm
                while remaining > 0 {
                    let block_len = min(remaining, 512 - (position % 512));
                    //debug!("Writing block: pos={}, src_pos={}, len={}", position, src_pos, block_len);

                    // Write data chunk
                    connection.outbound.pbytes(&data, src_pos, block_len);

                    position += block_len;
                    src_pos += block_len;
                    remaining -= block_len;

                    // Add delimiter if needed
                    if position % 512 == 0 && remaining > 0 {
                        //debug!("Adding delimiter at position {}", position);
                        connection.outbound.p1(255);
                        position += 1;
                    }
                }
            }

            Ok(())
        } else {
            Err("Invalid JS5 request".into())
        }
    }
}