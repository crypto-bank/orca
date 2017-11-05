// WebSocket streams client utilities.

use std::collections::HashMap;
use futures::future::{Future, Loop};
use futures::prelude::*;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::async::TcpStream;
use websocket::client::async::{Client as WebSocketClient, TlsStream};

use ::core::errors::*;
use ::currency::Pair;
use ::streams::{CmdReceiver, Event, EventSender};
use ::util::{reactor, boxfuture, FutureExt};


/// Parsed WebSocket stream message.
pub struct Message {
    /// Sequence ID. This field is optional.
    pub seq_id: i64,
    /// WebSocket channel ID.
    pub chan_id: i64,
    /// Parsed message events.
    pub events: Vec<Event>,
}

/// TLS-encrypted asynchronous WebSocket stream client.
pub type Client = WebSocketClient<TlsStream<TcpStream>>;

/// WebSocket Stream loop state.
pub type LoopState = (Client, Option<Handle>);

/// WebSocket Stream loop future.
pub type LoopFuture = BoxFuture<Loop<(), LoopState>>;

/// WebSocket Stream connection handlers.
pub struct Handle {
    pub reactor: reactor::Handle,
    pub sender: EventSender,
    pub commands: CmdReceiver,
    pub pairs: HashMap<i64, Pair>,
}

impl Handle {
    /// Creates new connection struct.
    pub fn new(sender: EventSender, commands: CmdReceiver, handle: reactor::Handle) -> Self {
        Handle {
            reactor: handle,
            sender: sender,
            commands: commands,
            pairs: HashMap::new(),
        }
    }

    /// Returns currency pair by channel id.
    pub fn pair(&self, id: &i64) -> Option<Pair> {
        match self.pairs.get(&id) {
            Some(pair) => Some(pair.clone()),
            None => None,
        }
    }

    /// Creates handle with new channel.
    pub fn with_channel(self, id: i64, pair: Pair) -> Self {
        let mut pairs = self.pairs.clone();
        pairs.insert(id, pair);
        Handle {
            pairs: pairs,
            ..self
        }
    }
}

/// Asynchronously Connects to WebSocket stream.
pub fn connect(address: &str, handle: &reactor::Handle) -> BoxFuture<Client> {
    ClientBuilder::new(address)
        .unwrap() // panics on address parse
        .async_connect_secure(None, handle)
        .map(|(duplex, _)| duplex)
        .map_err(|e| e.into())
        .into_box()
}

/// Returns future that breaks WebSocket stream loop.
pub fn break_loop() -> LoopFuture {
    boxfuture::ok(Loop::Break(()))
}

/// Returns future that continues WebSocket stream loop.
pub fn continue_loop(stream: Client, handle: Handle) -> LoopFuture {
    boxfuture::ok(Loop::Continue((stream, Some(handle))))
}

/// Sends message to a WebSocket stream and continues loop.
pub fn send_msg(msg: OwnedMessage, stream: Client, handle: Handle) -> LoopFuture {
    stream
        .send(msg)
        .map(|stream| Loop::Continue((stream, Some(handle))))
        .map_err(|e| e.into())
        .into_box()
}
