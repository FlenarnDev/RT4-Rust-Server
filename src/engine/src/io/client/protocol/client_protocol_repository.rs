use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;
use crate::entity::network_player::NetworkPlayer;
use crate::io::client::codec::event_applet_focus_decoder::EventAppletFocusDecoder;
use crate::io::client::codec::event_camera_position_decoder::EventCameraPositionDecoder;
use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::codec::verification_decoder::VerificationDecoder;
use crate::io::client::codec::window_status_decoder::WindowStatusDecoder;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::handler::verification_handler::VerificationHandler;
use crate::io::client::handler::window_status_handler::WindowStatusHandler;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::model::event_applet_focus::EventAppletFocusMessage;
use crate::io::client::protocol::client_protocol::ClientProtocol;
use crate::io::packet::Packet;

#[derive(Debug)]
struct RepositoryError(String);

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait MessageDecoderErasure: Send + Sync {
    fn protocol(&self) -> &ClientProtocol;
    fn decode_erased(&self, packet: &mut Packet, length: usize) -> Box<dyn IncomingMessage + Send + Sync>;
}

pub trait MessageHandlerErasure: Send + Sync {
    fn handle_erased(&self, message: &(dyn IncomingMessage + Send + Sync), network_player: &mut NetworkPlayer) -> bool;
}

pub struct NoopHandler<M> {
    _phantom: std::marker::PhantomData<M>,
}

impl<M> NoopHandler<M> {
    pub fn new() -> Self {
        NoopHandler {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M> MessageHandler for NoopHandler<M>
where
    M: IncomingMessage + Send + Sync + 'static,
{
    type Message = M;

    fn handle(&self, _message: &Self::Message, _network_player: &mut NetworkPlayer) -> bool {
        false // Do nothing, just return false
    }
}

impl<D, M> MessageDecoderErasure for D
where
    D: MessageDecoder<Message = M> + Send + Sync,
    M: IncomingMessage + Send + Sync + 'static
{
    fn protocol(&self) -> &ClientProtocol {
        self.protocol()
    }

    fn decode_erased(&self, packet: &mut Packet, length: usize) -> Box<dyn IncomingMessage + Send + Sync> {
        self.decode(packet, length)
    }
}

impl<H, M> MessageHandlerErasure for H
where
    H: MessageHandler<Message = M> + Send + Sync,
    M: IncomingMessage + Send + Sync + 'static
{
    fn handle_erased(&self, message: &(dyn IncomingMessage + Send + Sync), network_player: &mut NetworkPlayer) -> bool {
        if let Some(typed_message) = message.as_any().downcast_ref::<M>() {
            self.handle(typed_message, network_player)
        } else {
            false
        }
    }
}

type DecoderBox = Box<dyn MessageDecoderErasure>;
type HandlerBox = Box<dyn MessageHandlerErasure>;

pub struct ClientProtocolRepository {
    decoders: HashMap<u32, DecoderBox>,
    handlers: HashMap<u32, HandlerBox>,
}

impl ClientProtocolRepository {
    fn bind<M, D, H>(
        &mut self,
        decoder: D,
        handler: H,
    ) -> Result<(), RepositoryError>
    where
        M: IncomingMessage + Send + Sync + 'static,
        D: MessageDecoder<Message = M> + Send + Sync + 'static,
        H: MessageHandler<Message = M> + Send + Sync + 'static
    {
        let protocol_id = decoder.protocol().id;

        if self.decoders.contains_key(&protocol_id) {
            return Err(RepositoryError(format!("[ClientProtocolRepository] Already defined a {}", protocol_id)));
        }

        self.decoders.insert(protocol_id, Box::new(decoder));
        self.handlers.insert(protocol_id, Box::new(handler));

        Ok(())
    }

    fn bind_decoder_only<M, D>(
        &mut self,
        decoder: D,
    ) -> Result<(), RepositoryError>
    where
        M: IncomingMessage + Send + Sync + 'static,
        D: MessageDecoder<Message = M> + Send + Sync + 'static,
    {
        let protocol_id = decoder.protocol().id;

        if self.decoders.contains_key(&protocol_id) {
            return Err(RepositoryError(format!("[ClientProtocolRepository] Already defined a {}", protocol_id)));
        }

        // Create a NoopHandler for this message type
        let noop_handler = NoopHandler::<M>::new();

        self.decoders.insert(protocol_id, Box::new(decoder));
        self.handlers.insert(protocol_id, Box::new(noop_handler));

        Ok(())
    }

    pub fn new() -> Self {
        let mut repository = ClientProtocolRepository {
            decoders: HashMap::new(),
            handlers: HashMap::new()
        };

        repository.bind(
            WindowStatusDecoder,
            WindowStatusHandler,
        ).unwrap();
        
        repository.bind(
            VerificationDecoder,
            VerificationHandler,
        ).unwrap();
        
        repository.bind_decoder_only::<_, EventCameraPositionDecoder>(
            EventCameraPositionDecoder,
        ).unwrap();

        repository.bind_decoder_only::<_, EventAppletFocusDecoder>(
            EventAppletFocusDecoder
        ).unwrap();
        
        repository
    }

    pub fn get_local_decoder(&self, protocol: &ClientProtocol) -> Option<&dyn MessageDecoderErasure> {
        self.decoders.get(&protocol.id).map(|boxed| boxed.as_ref())
    }

    pub fn get_local_handler(&self, protocol: &ClientProtocol) -> Option<&dyn MessageHandlerErasure> {
        self.handlers.get(&protocol.id).map(|boxed| boxed.as_ref())
    }
}

lazy_static! {
    static ref CLIENT_PROTOCOL_REPOSITORY: ClientProtocolRepository = ClientProtocolRepository::new();
}

pub fn get_decoder(protocol: &ClientProtocol) -> Option<&dyn MessageDecoderErasure> {
    CLIENT_PROTOCOL_REPOSITORY.get_local_decoder(protocol)
}

pub fn get_handler(protocol: &ClientProtocol) -> Option<&dyn MessageHandlerErasure> {
    CLIENT_PROTOCOL_REPOSITORY.get_local_handler(protocol)
}