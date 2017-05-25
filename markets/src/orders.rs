//! Market orders.

use std::convert::{From, Into};

use orca_currency::{Pair, Volume};
use super::RawTrade;

/// Order kind can be either `Ask` when we are selling
/// or a `Bid` when we are trying to buy something.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderKind {
    /// Ask or a sell.
    Ask,
    /// Bid or a buy.
    Bid,
}

impl From<i64> for OrderKind {
    fn from(i: i64) -> Self {
        match i {
            0 => OrderKind::Ask,
            1 => OrderKind::Bid,
            _ => unimplemented!(),
        }
    }
}

impl Into<i64> for OrderKind {
    fn into(self) -> i64 {
        match self {
            OrderKind::Ask => 0,
            OrderKind::Bid => 1,
        }
    }
}

/// Currency trade order.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Order {
    /// Order kind, which is an `Ask` or a `Bid`.
    pub kind: OrderKind,

    /// Price rate is a price of a unit.
    pub rate: Volume,

    /// Order volume.
    pub volume: Volume,
}

impl Order {
    /// Currency pair.
    #[inline]
    pub fn pair(&self) -> Pair {
        Pair(self.rate.currency(), self.volume.currency())
    }

    /// Total price of an order.
    #[inline]
    pub fn price(&self) -> f64 {
        self.volume.amount() * self.rate.amount()
    }

    /// Total price of an order in Volume struct.
    #[inline]
    pub fn price_volume(&self) -> Volume {
        Volume(self.rate.currency(), self.price())
    }
}

impl Into<RawOrder> for Order {
    fn into(self) -> RawOrder {
        RawOrder {
            kind: self.kind,
            rate: self.rate.amount(),
            volume: self.volume.amount(),
        }
    }
}

/// Raw currency order.
/// 
/// It's format used in databases and streams,
/// in places where  currency pair is always known.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RawOrder {
    /// Order kind, which is an `Ask` or a `Bid`.
    pub kind: OrderKind,

    /// Price rate is a price of a unit.
    pub rate: f64,

    /// Order volume.
    pub volume: f64,
}

impl RawOrder {
    /// Total price of an order.
    #[inline]
    pub fn price(&self) -> f64 {
        self.volume * self.rate
    }

    /// Creates order with known currency pair.
    #[inline]
    pub fn to_order(&self, pair: &Pair) -> Order {
        Order {
            kind: self.kind,
            rate: Volume(pair.0, self.rate),
            volume: Volume(pair.1, self.volume),
        }
    }
}

/// Order making strategy.
pub enum OrderStrategy {
    // You may optionally set "fillOrKill", "immediateOrCancel", "postOnly" to 1. 
    /// A fill-or-kill order will either fill in its entirety or be completely aborted.
    FillOrKill,
    
    /// An immediate-or-cancel order can be partially or completely filled,
    /// but any portion of the order that cannot be filled immediately will be canceled
    /// rather than left on the order book
    ImmediateOrCancel,

    /// A post-only order will only be placed if no portion of it fills immediately;
    /// this guarantees you will never pay the taker fee on any part of the order that fills.
    PostOnly,
}

/// Result of placing order on a market.
pub enum OrderResult {
    /// Order was placed, inside value is ID.
    Placed(i64),

    /// Order resulted in trades on place and order was filled.
    Filled(Vec<RawTrade>),

    /// Order resulted in trades on place and was partially filled.
    PartiallyFilled(i64, Vec<RawTrade>),
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::{Order, OrderKind, RawOrder};
    use std::str::{FromStr};
    use orca_currency::{Volume, Pair, Symbol};
    
    #[test]
    fn order_to_json() {
        let order = Order{
            kind:   OrderKind::Ask, // we want to buy
            rate:   Volume(Symbol::USD, 946.0), // USD price of each BTC
            volume: Volume(Symbol::BTC, 0.5),   // Amount of BTC in order
        };
        assert_eq!(serde_json::to_string(&order).unwrap(),
            "{\"kind\":\"Ask\",\"rate\":[\"USD\",946.0],\"volume\":[\"BTC\",0.5]}");
    }

    #[test]
    fn raw_to_order() {
        let raw_order = RawOrder{
            kind:   OrderKind::Ask,
            rate:   0.0003,
            volume: 10000.0,
        };
        let pair = Pair::from_str("BTC_XRP").unwrap();
        let order = raw_order.to_order(&pair);
        assert_eq!(order, Order{
            kind:   OrderKind::Ask,
            rate:   Volume(Symbol::BTC, 0.0003),
            volume: Volume(Symbol::XRP, 10000.0),
        });
    }

    #[test]
    fn order_to_raw() {
        let order = Order{
            kind:   OrderKind::Ask,
            rate:   Volume(Symbol::BTC, 0.0003),
            volume: Volume(Symbol::XRP, 10000.0),
        };
        let raw_order: RawOrder = order.into();
        assert_eq!(raw_order, RawOrder{
            kind:   OrderKind::Ask,
            rate:   0.0003,
            volume: 10000.0,
        });
    }
}
