use std::collections::HashMap;
use std::fmt;
use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;

#[derive(Debug)]
struct RepositoryError(String);

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub struct ClientProtocolRepository {
    decoders: HashMap<u32, Box<dyn MessageDecoder>>,
    handlers: HashMap<u32, Box<dyn MessageHandler<dyn IncomingMessage>>>,
}

impl ClientProtocolRepository {
    fn bind(
        &mut self,
        decoder: Box<dyn MessageDecoder>,
        handler: Option<Box<dyn MessageHandler<dyn IncomingMessage>>>
    ) -> Result<(), RepositoryError> {
        let protocol_id = decoder.protocol().id;
        
        if self.decoders.contains_key(&protocol_id) {
            return Err(RepositoryError(format!("[ClientProtocolRepository] Already defined a {}", protocol_id)));
        }
        
        self.decoders.insert(protocol_id, decoder);
        
        if let Some(handler) = handler {
            self.handlers.insert(protocol_id, handler);
        }
        
        Ok(())
    }
    
    pub fn new() -> Self {
        let mut repository = ClientProtocolRepository {
            decoders: HashMap::new(),
            handlers: HashMap::new()
        };
        
        /*repository.bind(
            Box::
        )*/
        repository
    }

    pub fn get_decoder(&self, protocol: &ClientProtocol) -> Option<&Box<dyn MessageDecoder>> {
        self.decoders.get(&protocol.id)
    }
    pub fn get_handler(&self, protocol: &ClientProtocol) -> Option<&Box<dyn MessageHandler<dyn IncomingMessage>>> {
        self.handlers.get(&protocol.id)
    }
}