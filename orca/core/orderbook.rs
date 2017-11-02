
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use core::currency::CurrencyPair;

/// Ordered order books for asks and bids.
pub struct OrderBook {
    pub pair: CurrencyPair,
    pub asks: OrderSideBook,
    pub bids: OrderSideBook,
}

impl OrderBook {
    /// Creates new order book.
    pub fn new(pair: &CurrencyPair) -> Self {
        OrderBook {
            pair: pair.clone(),
            asks: OrderSideBook::new(),
            bids: OrderSideBook::new(),
        }
    }
}

/// Ordered order book for single side.
pub struct OrderSideBook {
    pub inner: BTreeMap<OrderedFloat<f64>, OrderedFloat<f64>>,
}

impl OrderSideBook {
    /// Creates new order book.
    pub fn new() -> Self {
        OrderSideBook { inner: BTreeMap::new() }
    }

    /// Sets volume on rate.
    pub fn set(&mut self, rate: f64, volume: f64) {
        self.inner.insert(OrderedFloat(rate), OrderedFloat(volume));
    }
}
