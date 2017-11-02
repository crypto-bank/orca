
use websocket::async::TcpStream;
use websocket::client::async::{Client as WebSocketClient, TlsStream};

/// TLS-encrypted asynchronous WebSocket stream client.
pub type Client = WebSocketClient<TlsStream<TcpStream>>;
