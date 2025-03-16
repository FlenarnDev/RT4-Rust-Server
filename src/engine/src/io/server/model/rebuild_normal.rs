use std::collections::{BTreeSet, HashSet};
use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::protocol::server_protocol_priority::ServerProtocolPriority;

#[derive(Debug)]
pub struct RebuildNormal {
    zone_x: i32,
    zone_z: i32,
    coord_x: i32,
    coord_z: i32,
}

impl RebuildNormal {
    pub fn new(zone_x: i32, zone_z: i32, coord_x: i32, coord_z: i32) -> RebuildNormal {
        RebuildNormal { zone_x, zone_z, coord_x, coord_z }
    }
    
    pub fn zone_x(&self) -> i32 { self.zone_x }
    
    pub fn zone_z(&self) -> i32 { self.zone_z }
    
    pub fn local_x(&self) -> i32 {self.coord_x }
    
    pub fn local_z(&self) -> i32 {self.coord_z }
    
    pub fn mapsquares(&self) -> BTreeSet<i32> {
        let min_x: i32 = self.zone_x - 6;
        let max_x: i32 = self.zone_x + 6;
        let min_z: i32 = self.zone_z - 6;
        let max_z: i32 = self.zone_z + 6;
        let mut result = BTreeSet::new();
        
        // Build area is 13x13 zones (8*13 = 104 tiles), so we need to load 6 zones in each direction
        for x in min_x..=max_x {
            let mx: i32 = (x << 3) >> 6;
            for z in min_z..=max_z {
                let mz: i32 = (z << 3) >> 6;
                result.insert(mx << 8 | mz);
            }
        }
        
        result
    }
}

impl OutgoingMessage for RebuildNormal {
    fn priority(&self) -> ServerProtocolPriority {
        ServerProtocolPriority::IMMEDIATE
    }
}