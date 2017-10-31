
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use core::currency::CurrencyPair;

/// Ordered order books.
pub struct OrderBooks {
    pub pair: CurrencyPair,
    pub asks: OrderBook,
    pub bids: OrderBook,
}

impl OrderBooks {
    /// Creates new order book.
    pub fn new(pair: CurrencyPair) -> Self {
        OrderBooks {
            pair: pair,
            asks: OrderBook::new(),
            bids: OrderBook::new(),
        }
    }
}

/// Ordered order book.
pub struct OrderBook {
    pub inner: BTreeMap<OrderedFloat<f64>, OrderedFloat<f64>>,
}

impl OrderBook {
    /// Creates new order book.
    pub fn new() -> Self {
        OrderBook { inner: BTreeMap::new() }
    }

    /// Sets volume on rate.
    pub fn set(&mut self, rate: f64, volume: f64) {
        self.inner.insert(OrderedFloat(rate), OrderedFloat(volume));
    }
}
