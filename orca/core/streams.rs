//! Markets streams.

use tokio_core::reactor::Handle;
use core::errors::*;
use core::{CurrencyPair, OrderBooks, RawOrder, RawTrade};

/// Stream event.
pub enum Event {
    /// Message from stream.
    Message(Message),

    /// Close of a stream.
    Close,
}

/// Message from stream.
pub enum Message {
    /// Single trade.
    Trade(RawTrade),

    /// Single order book change.
    Order(RawOrder),

    /// Order books message.
    OrderBooks(OrderBooks),
}

/// Connected stream.
pub trait Stream {
    /// Creates a new connected stream.
    fn connect(handle: &Handle) -> Box<Future<Self>>;

    /// Subscribes to a currency pair.
    fn subscribe(&mut self, pair: &CurrencyPair) -> Future<()>;

    /// Unsubscribes from a currency pair.
    fn unsubscribe(&mut self, pair: &CurrencyPair) -> Future<()>;
}
