//! Markets streams.

use tokio_core::reactor::Handle;
use core::{CurrencyPair, OrderBook, RawOrder, RawTrade};

/// Message from stream.
pub enum Message {
    /// Single trade.
    Trade(RawTrade),

    /// Single order book change.
    Order(RawOrder),

    /// Order book message.
    /// Resets entire order book.
    OrderBook(OrderBook),
}

/// Stream event.
pub enum Event {
    /// Message from stream.
    Message(Message),

    /// Close of a stream.
    Close,
}

/// Stream future.
pub type Future<T> = ::futures::Future<Item=T, Error=errors::Error>;

/// Connected stream.
pub trait Stream {
    /// Creates a new connected stream.
    fn connect(handle: &Handle) -> Box<Future<Self>>;

    /// Subscribes to a currency pair.
    fn subscribe(&mut self, pair: &CurrencyPair) -> Future<()>;

    /// Unsubscribes from a currency pair.
    fn unsubscribe(&mut self, pair: &CurrencyPair) -> Future<()>;
}

pub mod errors {
    error_chain! {
        foreign_links {
            IoError(::std::io::Error);
            WebSocketError(::websocket::result::WebSocketError);
        }
    }
}

pub use self::errors::*;
