
use futures::Future;
use websocket::ClientBuilder;
use websocket::async::TcpStream;
use websocket::client::async::{Client as WebSocketClient, TlsStream};
use core::reactor;
use core::BoxFuture;
use utils::FutureExt;

/// TLS-encrypted asynchronous WebSocket stream client.
pub type Client = WebSocketClient<TlsStream<TcpStream>>;

/// Connects to WebSocket stream.
pub fn connect(address: &str, handle: &reactor::Handle) -> BoxFuture<Client> {
    ClientBuilder::new(address)
        .unwrap() // panics on address parse
        .async_connect_secure(None, handle)
        .map(|(duplex, _)| duplex)
        .map_err(|e| e.into())
        .into_box()
}
