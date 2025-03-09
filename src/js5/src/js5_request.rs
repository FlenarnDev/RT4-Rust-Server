use std::error::Error;
use log::debug;
use rs2cache::store::ARCHIVESET;
use cache::file_handler::{CACHE, MASTER_INDEX_VEC};
use io::connection::{write_and_clear_output, Connection};
use io::packet::Packet;

#[derive(Debug)]
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
    Invalid
}

impl Js5Request {
    
    pub async fn fulfill_request(connection: &mut Connection, request: &Js5Request) -> Result<(), Box<dyn Error>> {
        debug!("Fulfilling request: {:?}", request);
        
        if let Js5Request::Group { prefetch, archive, group } = request {
            
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
                connection.output.p1(*archive as i32);
                connection.output.p2(*group as i32);
                
                // TODO - The rest of serving is a bit fucked
            }

            write_and_clear_output(connection).await.expect("Failed to write and clear output packet while serving JS5 request");
            Ok(())
        } else {
            Err("Invalid JS5 request".into())
        }
    }
}