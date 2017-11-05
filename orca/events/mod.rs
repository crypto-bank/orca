//! Orca markets events.
// #SPC-markets-events

mod events;
mod convert;
pub use self::events::*;
pub use self::convert::*;

use ::markets::{Order, OrderBook, Trade};

/// Market event.
pub enum Event {
    /// Single trade.
    Trade(Trade),

    /// Single order book change.
    Order(Order),

    /// Order books message.
    OrderBook(OrderBook),
}

/// Events extension.
pub trait EventExt {
    /// Returns true if event is orderbook.
    fn is_order_book(&self) -> bool;
}

impl EventExt for Event {
    /// Returns true if event is orderbook.
    fn is_order_book(&self) -> bool {
        match self {
            &Event::OrderBook(_) => true,
            _ => false
        }
    }
}

impl EventExt for Vec<Event> {
    /// Returns true if vector size is `1` and first event is orderbook.
    fn is_order_book(&self) -> bool {
        match self.len() {
            1 => self[0].is_order_book(),
            _ => false
        }
    }
}
