use std::collections::HashMap;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::protocol::client_protocol::ClientProtocol;

pub struct ClientProtocolRepository {
    // HashMap of protocol -> handler
    handlers: HashMap<ClientProtocol, Box<dyn MessageHandler<dyn std::any::Any>>>,

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

    /// Bind a protocol to a callback and optional handler
    fn bind(
        &mut self,
        prot: ClientProtocol,
        callback: Box<dyn Fn(Vec<u8>) -> Option<Box<dyn std::any::Any>>>,
        handler: Option<Box<dyn MessageHandler<dyn std::any::Any>>>,
    ) {
        if let Some(handler) = handler {
            self.handlers.insert(prot.clone(), handler);
        }
        self.binds.insert(prot, callback);
    }

    /// Retrieve a handler for a protocol
    pub fn get_handler(&self, client_protocol: ClientProtocol) -> Option<&Box<dyn MessageHandler<dyn std::any::Any>>> {
        self.handlers.get(&client_protocol)
    }

    /// Retrieve a message from bytes for a protocol
    pub fn get_message(&self, client_protocol: ClientProtocol, bytes: Vec<u8>) -> Option<Box<dyn std::any::Any>> {
        self.binds.get(&client_protocol).map(|callback| callback(bytes))?
    }
}