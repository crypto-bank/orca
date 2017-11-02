//! Markets streams.

use core::{CurrencyPair, OrderBook, RawOrder, RawTrade, Market};

/// Market stream event.
pub enum Event {
    /// Single trade.
    Trade(RawTrade),

    /// Single order book change.
    Order(RawOrder),

    /// Order books message.
    OrderBook(OrderBook),
}

/// Market stream events.
pub type Events = (Market, CurrencyPair, Vec<Event>);

/// Market stream events sender.
pub type EventSender = ::futures::sync::mpsc::UnboundedSender<Events>;

/// Market stream events receiver.
pub type EventReceiver = ::futures::sync::mpsc::UnboundedReceiver<Events>;
