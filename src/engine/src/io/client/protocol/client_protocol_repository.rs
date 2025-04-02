use std::collections::HashMap;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::protocol::client_protocol::ClientProtocol;

pub struct ClientProtocolRepository {
    // HashMap of protocol -> handler
    handlers: HashMap<ClientProtocol, Box<dyn MessageHandler>>,

    // HashMap of protocol -> callback function (converts bytes to messages)
    binds: HashMap<ClientProtocol, Box<dyn Fn(Vec<u8>) -> Option<Box<dyn std::any::Any>>>>,
}

impl ClientProtocolRepository {
    pub fn new() -> Self {
        let mut repository = ClientProtocolRepository {
            handlers: HashMap::new(),
            binds: HashMap::new(),
        };

        // TODO - binds

        repository
    }

    pub fn get() -> &'static mut ClientProtocolRepository {
        unsafe {
            match &mut crate::engine::CLIENT_PROTOCOL_REPOSITORY {
                Some(client_protocol_repository) => client_protocol_repository,
                None => {
                    crate::engine::CLIENT_PROTOCOL_REPOSITORY.as_mut().unwrap()
                }
            }
        }
    }

    /// Bind a protocol to a callback and optional handler
    pub fn bind(&mut self, prot: ClientProtocol, handler: Box<dyn MessageHandler>) {
        self.handlers.insert(prot, handler);
    }

    /// Retrieve a handler for a protocol
    pub fn get_handler(&self, prot: ClientProtocol) -> Option<&dyn MessageHandler> {
        self.handlers.get(&prot).map(|h| h.as_ref()) // Return a reference to the trait object
    }

    /// Retrieve a message from bytes for a protocol
    pub fn get_message(&self, client_protocol: ClientProtocol, bytes: Vec<u8>) -> Option<Box<dyn std::any::Any>> {
        self.binds.get(&client_protocol).map(|callback| callback(bytes))?
    }
}