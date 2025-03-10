use std::cmp::min;
use std::error::Error;
use std::thread::sleep;
use log::debug;
use rs2cache::store::ARCHIVESET;
use cache::file_handler::{CACHE, MASTER_INDEX_VEC};
use cache::version_trailer::VersionTrailer;
use io::connection::{write_and_clear_output, Connection};
use io::packet::Packet;

const BLOCK_SIZE: usize = 512;
pub const BLOCK_HEADER_SIZE: usize = 1 + 2 + 1;
pub const BLOCK_DELIMITER_SIZE: usize = 1;
pub const BYTES_BEFORE_BLOCK: usize = BLOCK_SIZE - BLOCK_HEADER_SIZE;
pub const BYTES_AFTER_BLOCK: usize = BLOCK_SIZE - BLOCK_DELIMITER_SIZE;

#[derive(Debug)]
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
    Invalid
}

impl Js5Request {
    
    pub async fn fulfill_request(connection: &mut Connection, request: &Js5Request) -> Result<(), Box<dyn Error>> {
        debug!("Fulfilling request: {:?}", request);
        
        if let Js5Request::Group { urgent, archive, group } = request {
            
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
                //* Read data from cache into the packet
                let data = cache.store.read(*archive, *group as u32)?;
                let length = 2 + data.len() + (512 + data.len()) / 511;
                //debug!("Length of data packet: {}", data.len());
                //debug!("Data packet1: {:?}", data);
                //debug!("Estimated length of final packet: {}", length);

                let mut data_packet = Packet::from(data);
                connection.output = Packet::new(length);


                // TODO - is this needed?
                /*if *archive != ARCHIVESET {
                    debug!("Stripping version trailer from archive group");
                    VersionTrailer::strip(&mut data_packet.data);
                }*/

                connection.output.p1(*archive as i32);
                connection.output.p2(*group as i32);

                let mut compression= data_packet.data[0];
                if *urgent {
                    compression |= 0x80;
                }
                connection.output.p1(compression as i32);
                //debug!("compression: {}", compression);
                //debug!("remaining data: {}", data_packet.remaining());
                let data_to_write = data_packet.gbytes(min(data_packet.remaining() as usize, 508));
                //debug!("data to write: {:?}", data_to_write);
                connection.output.pbytes(&*data_to_write, 0, data_to_write.len());
                //debug!("Data packet2: {:?}", connection.output.data);
                
                
                //debug!("Wrote compression");
                
                while data_packet.remaining() > 0 {
                    connection.output.p1(255);
                    let data_to_add = data_packet.gbytes(min(data_packet.remaining() as usize, BYTES_AFTER_BLOCK));
                    connection.output.pbytes(&*data_to_add, 0, data_to_add.len());
                    //debug!("in while loop");
                }

                //debug!("Wrote data length: {}", connection.output.data.len());
                //debug!("Wrote data: {:?}", connection.output.data);
            }

            Ok(())
        } else {
            Err("Invalid JS5 request".into())
        }
    }
}