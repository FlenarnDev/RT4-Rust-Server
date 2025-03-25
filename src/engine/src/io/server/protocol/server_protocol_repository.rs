use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::io::server::codec::if_opensub_encoder::If_OpenSubEncoder;
use crate::io::server::codec::if_opentop_encoder::If_OpenTopEncoder;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::codec::rebuild_normal_encoder::RebuildNormalEncoder;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::outgoing_message::OutgoingMessage;
use crate::io::server::protocol::server_protocol::ServerProtocol;

struct TypedEncoder<T: OutgoingMessage> {
    encoder: Box<dyn MessageEncoder<T> + Send + Sync>,
}

pub struct ServerProtocolRepository {
    encoders: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    // Cache protocols by message type for faster lookup
    protocol_cache: HashMap<TypeId, ServerProtocol>,
}

impl ServerProtocolRepository {
    pub fn new() -> Self {
        let mut repository = ServerProtocolRepository {
            encoders: HashMap::new(),
            protocol_cache: HashMap::new(),
        };

        // Pre-register known encoders
        repository.bind::<RebuildNormal>(RebuildNormalEncoder::new());
        repository.bind::<If_OpenTop>(If_OpenTopEncoder::new());
        repository.bind::<If_OpenSub>(If_OpenSubEncoder::new());

        repository
    }

    #[inline]
    fn bind<T: 'static + OutgoingMessage>(&mut self, encoder: impl MessageEncoder<T> + 'static + Send + Sync) {
        let type_id = TypeId::of::<T>();

        if self.encoders.contains_key(&type_id) {
            panic!("Duplicate encoder encountered for TypeId: {:?}", type_id);
        }

        // Cache the protocol for faster lookups
        let protocol = encoder.protocol();
        self.protocol_cache.insert(type_id, protocol);

        // Store the typed encoder
        let typed_encoder = TypedEncoder {
            encoder: Box::new(encoder),
        };
        self.encoders.insert(type_id, Box::new(typed_encoder));
    }

    #[inline]
    pub fn get_encoder<T: 'static + OutgoingMessage>(&self, _: &T) -> Option<&(dyn MessageEncoder<T> + Send + Sync)> {
        let type_id = TypeId::of::<T>();

        // Fast path using direct lookup by TypeId
        self.encoders.get(&type_id)
            .and_then(|box_any| box_any.downcast_ref::<TypedEncoder<T>>())
            .map(|typed| typed.encoder.as_ref())
    }

    #[inline]
    pub fn get_protocol<T: 'static + OutgoingMessage>(&self, _: &T) -> Option<ServerProtocol> {
        // Direct lookup in protocol cache
        self.protocol_cache.get(&TypeId::of::<T>()).cloned()
    }
}

// Use lazy_static for global singleton access
lazy_static::lazy_static! {
    pub static ref SERVER_PROTOCOL_REPOSITORY: ServerProtocolRepository = {
        // Initialize the repository with all known encoders
        ServerProtocolRepository::new()
    };
}