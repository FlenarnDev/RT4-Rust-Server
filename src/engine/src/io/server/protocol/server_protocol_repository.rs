use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::io::server::codec::if_opensub_encoder::If_OpenSubEncoder;
use crate::io::server::codec::if_opentop_encoder::If_OpenTopEncoder;
use crate::io::server::codec::message_encoder::MessageEncoder;
use crate::io::server::codec::rebuild_normal_encoder::RebuildNormalEncoder;
use crate::io::server::model::if_opensub::If_OpenSub;
use crate::io::server::model::if_opentop::If_OpenTop;
use crate::io::server::model::rebuild_normal::RebuildNormal;
use crate::io::server::outgoing_message::OutgoingMessage;

struct TypedEncoder<T: OutgoingMessage> {
    encoder: Box<dyn MessageEncoder<T> + Send + Sync>,
}

pub struct ServerProtocolRepository {
    encoders: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServerProtocolRepository {
    pub fn new() -> Self {
        let mut repository = ServerProtocolRepository {
            encoders: HashMap::new(),
        };
        
        repository.bind::<RebuildNormal>(RebuildNormalEncoder::new());
        repository.bind::<If_OpenTop>(If_OpenTopEncoder::new());
        repository.bind::<If_OpenSub>(If_OpenSubEncoder::new());
        
        repository
    }

    fn bind<T: 'static + OutgoingMessage>(&mut self, encoder: impl MessageEncoder<T> + 'static + Send + Sync) {
        let type_id = TypeId::of::<T>();
        
        if self.encoders.contains_key(&type_id) {
            panic!("Duplicate encoder encountered.");
        }
        
        let typed_encoder = TypedEncoder {
            encoder: Box::new(encoder),
        };
        self.encoders.insert(type_id, Box::new(typed_encoder));
    }

    pub fn get_encoder<T: 'static + OutgoingMessage>(&self, _: &T) -> Option<&(dyn MessageEncoder<T> + Send + Sync)> {
        let type_id = TypeId::of::<T>();
        self.encoders.get(&type_id)
            .and_then(|box_any| box_any.downcast_ref::<TypedEncoder<T>>())
            .map(|typed| typed.encoder.as_ref())
    }
}

lazy_static::lazy_static! {
    pub static ref SERVER_PROTOCOL_REPOSITORY: ServerProtocolRepository = ServerProtocolRepository::new();
}