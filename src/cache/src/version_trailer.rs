// Unnecessary?

pub struct VersionTrailer;

impl VersionTrailer {
    /// Peek at the version without modifying the packet's position
    pub fn peek(data: &mut Vec<u8>) -> Option<u16> {
        if data.len() >= 2 {
            let index = data.len() - 2;
            let version = ((data[index] as u16 & 0xFF) << 8) | 
                (data[index + 1] as u16 & 0xFF);
            Some(version)
        } else {
            None
        }
    }
    
    /// Strip the version from the `data` and return it
    pub fn strip(data: &mut Vec<u8>) -> Option<u16> {
        if data.len() >= 2 {
            let index = data.len() - 2;
            let version = ((data[index] as u16 & 0xFF) << 8) | 
                (data[index + 1] as u16 & 0xFF);
            
            data.truncate(index);
            
            Some(version)
        } else {
            None
        }
    }
}