//! Market events.

use orca_currency::Pair;
use super::{Trade, RawTrade, Order, OrderBook};

/// Market event.
pub enum Event {
    /// Order book update.
    Order(Order),
    /// Entire order book update.
    OrderBook(OrderBook),
    /// New trade.
    Trade(Trade),
    /// New or historical trades.
    Trades(Pair, Vec<RawTrade>),
}
