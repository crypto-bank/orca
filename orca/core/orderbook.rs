
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use core::currency::CurrencyPair;

/// Ordered order books for asks and bids.
pub struct OrderBook {
    pub pair: CurrencyPair,
    pub asks: Book,
    pub bids: Book,
}

impl OrderBook {
    /// Creates new order book.
    pub fn new(pair: &CurrencyPair) -> Self {
        OrderBook {
            pair: pair.clone(),
            asks: Book::new(),
            bids: Book::new(),
        }
    }
}

/// Ordered order book for single side.
pub struct Book {
    pub inner: BTreeMap<OrderedFloat<f64>, OrderedFloat<f64>>,
}

impl Book {
    /// Creates new order book.
    pub fn new() -> Self {
        Book { inner: BTreeMap::new() }
    }

    /// Sets volume on rate.
    pub fn set(&mut self, rate: f64, volume: f64) {
        self.inner.insert(OrderedFloat(rate), OrderedFloat(volume));
    }
}
