//! Market trades.

use std::convert::Into;
use orca_currency::Pair;
use super::{Order, RawOrder};


/// Currency trade.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Trade {
    /// Trade ID, unique only for a market.
    pub id: i64,

    /// Trade order.
    pub order: Order,

    /// Trade timestamp.
    pub timestamp: i64,
}

impl Trade {
    /// Currency pair.
    #[inline]
    pub fn pair(&self) -> Pair {
        self.order.pair()
    }
}

impl Into<RawTrade> for Trade {
    fn into(self) -> RawTrade {
        RawTrade {
            id: self.id,
            order: self.order.into(),
            timestamp: self.timestamp,
        }
    }
}


/// Raw currency trade.
/// 
/// It's format used in databases and streams,
/// in places where  currency pair is always known.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RawTrade {
    /// Trade ID, unique only for a market.
    pub id: i64,

    /// Trade order.
    pub order: RawOrder,

    /// Trade timestamp.
    pub timestamp: i64,
}

impl RawTrade {
    /// Converts raw trade to a trade with known currency pair.
    pub fn to_trade(&self, pair: &Pair) -> Trade {
        Trade {
            id: self.id,
            order: self.order.to_order(pair),
            timestamp: self.timestamp,
        }
    }
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::{Order, OrderKind};
    use orca_currency::{Symbol, Volume};

    #[test]
    fn trade_to_json() {
        let order = Order{
            kind:   OrderKind::Bid,
            volume: Volume(Symbol::LTC, 0.47170812),
            rate:   Volume(Symbol::BTC, 0.008478980),
        };
        let trade = Trade{
            id:        4050895,
            order:     order,
            timestamp: 1492641157,
        };
        assert_eq!(serde_json::to_string(&trade).unwrap(),
            "{\"id\":4050895,\"order\":{\"kind\":\"Bid\",\"rate\":[\"BTC\",0.00847898],\"volume\":[\"LTC\",0.47170812]},\"timestamp\":1492641157}");
    }
}
