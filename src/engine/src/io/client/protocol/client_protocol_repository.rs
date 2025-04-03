use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::entity::player::Player;
use crate::io::client::codec::event_applet_focus_decoder::EventAppletFocusDecoder;
use crate::io::client::codec::event_camera_position_decoder::EventCameraPositionDecoder;
use crate::io::client::codec::message_decoder::MessageDecoder;
use crate::io::client::codec::verification_decoder::VerificationDecoder;
use crate::io::client::codec::window_status_decoder::WindowStatusDecoder;
use crate::io::client::handler::message_handler::MessageHandler;
use crate::io::client::handler::verification_handler::VerificationHandler;
use crate::io::client::handler::window_status_handler::WindowStatusHandler;
use crate::io::client::incoming_message::IncomingMessage;
use crate::io::client::protocol::client_protocol::{ClientProtocol, ProtocolId};
use crate::io::packet::Packet;

#[derive(Debug)]
struct RepositoryError(String);

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RepositoryError {}

// Update the trait bound to include Send + Sync
pub trait MessageDecoderErasure: Send + Sync {
    fn protocol(&self) -> &ClientProtocol;
    fn decode_erased(&self, packet: &mut Packet, length: usize) -> Box<dyn IncomingMessage + Send + Sync>;
}

// Update the trait bound to include Send + Sync
pub trait MessageHandlerErasure: Send + Sync {
    fn handle_erased(&self, message: &(dyn IncomingMessage + Send + Sync), player: &mut Player) -> bool;
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

    fn handle(&self, _message: &Self::Message, _player: &mut Player) -> bool {
        false
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
    fn handle_erased(&self, message: &(dyn IncomingMessage + Send + Sync), player: &mut Player) -> bool {
        // Using trait upcasting if supported, otherwise fallback to downcast
        if let Some(typed_message) = message.as_any().downcast_ref::<M>() {
            self.handle(typed_message, player)
        } else {
            false
        }
    }
}

// Define the type aliases with explicit Send + Sync bounds
type DecoderBox = Arc<dyn MessageDecoderErasure>;
type HandlerBox = Arc<dyn MessageHandlerErasure>;

pub struct ClientProtocolRepository {
    decoders: HashMap<ProtocolId, DecoderBox>,
    handlers: HashMap<ProtocolId, HandlerBox>,
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
            return Err(RepositoryError(format!("[ClientProtocolRepository] Already defined a protocol with ID: {:?}", protocol_id)));
        }

        self.decoders.insert(protocol_id, Arc::new(decoder));
        self.handlers.insert(protocol_id, Arc::new(handler));

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
            return Err(RepositoryError(format!("[ClientProtocolRepository] Already defined a protocol with ID: {:?}", protocol_id)));
        }

        // Create a NoopHandler for this message type
        let noop_handler = NoopHandler::<M>::new();

        self.decoders.insert(protocol_id, Arc::new(decoder));
        self.handlers.insert(protocol_id, Arc::new(noop_handler));

        Ok(())
    }

    pub fn new() -> Self {
        let mut repository = ClientProtocolRepository {
            decoders: HashMap::new(),
            handlers: HashMap::new()
        };

        // Using a macro or function to reduce code duplication
        macro_rules! register_protocol {
            ($decoder:expr, $handler:expr) => {
                if let Err(e) = repository.bind($decoder, $handler) {
                    // Using proper logging instead of panicking
                    log::warn!("Failed to register protocol: {}", e);
                }
            };
            ($decoder:expr) => {
                if let Err(e) = repository.bind_decoder_only::<_, _>($decoder) {
                    log::warn!("Failed to register protocol: {}", e);
                }
            };
        }

        register_protocol!(WindowStatusDecoder, WindowStatusHandler);
        register_protocol!(VerificationDecoder, VerificationHandler);
        register_protocol!(EventCameraPositionDecoder);
        register_protocol!(EventAppletFocusDecoder);

        repository
    }

    pub fn get_decoder(&self, protocol_id: ProtocolId) -> Option<&dyn MessageDecoderErasure> {
        self.decoders.get(&protocol_id).map(|boxed| boxed.as_ref())
    }

    pub fn get_handler(&self, protocol_id: ProtocolId) -> Option<&dyn MessageHandlerErasure> {
        self.handlers.get(&protocol_id).map(|boxed| boxed.as_ref())
    }
}

lazy_static! {
    static ref CLIENT_PROTOCOL_REPOSITORY: ClientProtocolRepository = ClientProtocolRepository::new();
}

pub fn get_decoder(protocol: &ClientProtocol) -> Option<&dyn MessageDecoderErasure> {
    CLIENT_PROTOCOL_REPOSITORY.get_decoder(protocol.id)
}

pub fn get_handler(protocol: &ClientProtocol) -> Option<&dyn MessageHandlerErasure> {
    CLIENT_PROTOCOL_REPOSITORY.get_handler(protocol.id)
}