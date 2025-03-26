use std::any::{Any, TypeId};
use fnv::FnvHashMap;
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
    encoders: FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
    protocol_cache: FnvHashMap<TypeId, ServerProtocol>,
}

impl ServerProtocolRepository {
    pub fn new() -> Self {
        Self::builder()
            .with::<RebuildNormal>(RebuildNormalEncoder::new())
            .with::<If_OpenTop>(If_OpenTopEncoder::new())
            .with::<If_OpenSub>(If_OpenSubEncoder::new())
            .build()
    }

    pub fn builder() -> ServerProtocolRepositoryBuilder {
        ServerProtocolRepositoryBuilder::new()
    }

    #[inline]
    fn bind<T: 'static + OutgoingMessage>(&mut self, encoder: impl MessageEncoder<T> + 'static + Send + Sync) {
        let type_id = TypeId::of::<T>();

        if self.encoders.contains_key(&type_id) {
            panic!("Duplicate encoder encountered for TypeId: {:?}", type_id);
        }

        let protocol = encoder.protocol();
        self.protocol_cache.insert(type_id, protocol);

        let typed_encoder = TypedEncoder {
            encoder: Box::new(encoder),
        };

        self.encoders.insert(type_id, Box::new(typed_encoder));
    }

    #[inline]
    pub fn get_encoder<T: 'static + OutgoingMessage>(&self, _: &T) -> Option<&(dyn MessageEncoder<T> + Send + Sync)> {
        let type_id = TypeId::of::<T>();

        self.encoders.get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<TypedEncoder<T>>())
            .map(|typed| typed.encoder.as_ref())
    }

    #[inline]
    pub fn get_protocol<T: 'static + OutgoingMessage>(&self, _: &T) -> Option<ServerProtocol> {
        let type_id = TypeId::of::<T>();
        self.protocol_cache.get(&type_id).copied()
    }
}

pub struct ServerProtocolRepositoryBuilder {
    repository: ServerProtocolRepository,
}

impl ServerProtocolRepositoryBuilder {
    fn new() -> Self {
        Self {
            repository: ServerProtocolRepository {
                encoders: FnvHashMap::default(),
                protocol_cache: FnvHashMap::default(),
            }
        }
    }

    pub fn with<T: 'static + OutgoingMessage>(mut self, encoder: impl MessageEncoder<T> + 'static + Send + Sync) -> Self {
        self.repository.bind(encoder);
        self
    }

    pub fn build(self) -> ServerProtocolRepository {
        self.repository
    }
}

lazy_static::lazy_static! {
    pub static ref SERVER_PROTOCOL_REPOSITORY: ServerProtocolRepository = {
        ServerProtocolRepository::new()
    };
}