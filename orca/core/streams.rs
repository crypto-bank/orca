//! Markets streams.

use core::errors::*;
use core::{reactor, BoxFuture, CurrencyPair, OrderBooks, RawOrder, RawTrade};

/// Message from stream.
pub enum Message {
    /// Single trade.
    Trade(RawTrade),

    /// Single order book change.
    Order(RawOrder),

    /// Order books message.
    OrderBooks(OrderBooks),
}

pub trait Stream: Sized {
    /// Creates a connected stream..
    fn connect(handle: &reactor::Handle) -> BoxFuture<Self>;

    /// Closes a stream.
    fn close(&mut self) -> BoxFuture<()>;

    /// Subscribes to currency pair stream.
    fn subscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()>;

    /// Unsubscribes from currency pair stream.
    fn unsubscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()>;
}
