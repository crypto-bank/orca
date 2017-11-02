//! Orca core module.

#[cfg(proto3)]
pub mod currency;
#[cfg(proto3)]
pub mod market;
#[cfg(proto3)]
pub mod order;
#[cfg(proto3)]
pub mod trade;
pub mod errors;
mod orderbook;

pub use self::currency::*;
pub use self::market::*;
pub use self::order::*;
pub use self::trade::*;
pub use self::orderbook::*;
pub use self::errors::Future;
pub use self::errors::BoxFuture;

pub mod reactor {
    pub use tokio_core::reactor::*;
}

impl ::std::convert::TryFrom<i64> for OrderKind {
    type Error = errors::Error;

    fn try_from(k: i64) -> Result<Self, Self::Error> {
        match k {
            0 => Ok(OrderKind::Ask),
            1 => Ok(OrderKind::Bid),
            _ => Err(errors::ErrorKind::InvalidOrderKind(k).into()),
        }
    }
}

/// NOTE: this operation is pretty slow
impl<'a> ::std::convert::TryFrom<&'a str> for Currency {
    type Error = errors::Error;

    // TODO: it should use generated lookup table #bitfinex
    fn try_from(name: &str) -> Result<Self, Self::Error> {
        for c in <Currency as ::protobuf::ProtobufEnum>::values() {
            if <Currency as ::protobuf::ProtobufEnum>::descriptor(c).name() == name {
                return Ok(c.clone());
            }
        }
        Err(
            errors::ErrorKind::InvalidCurrencyPair(name.to_owned()).into(),
        )
    }
}

impl ::std::fmt::Display for Currency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(
            <Currency as ::protobuf::ProtobufEnum>::descriptor(self).name(),
        )
    }
}

impl ::std::fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}/{}", self.quote, self.base)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::convert::TryFrom;

    #[bench]
    fn currency_try_from_str(b: &mut Bencher) {
        b.iter(|| { Currency::try_from("BTC").unwrap(); });
    }

    #[bench]
    fn order_kind_try_from_i64(b: &mut Bencher) {
        b.iter(|| { OrderKind::try_from(0 as i64).unwrap(); });
    }
}


#[cfg(test)]
mod pbbench {
    use test::Bencher;
    use protobuf::Message;
    use protobuf::parse_from_bytes;
    use core::{OrderKind, RawOrder, RawTrade};

    fn create_order() -> RawOrder {
        let mut order = RawOrder::new();
        order.set_kind(OrderKind::Bid);
        order.set_rate(0.00002854);
        order.set_volume(175.19271198);
        order
    }

    fn create_trade() -> RawTrade {
        let mut trade = RawTrade::new();
        trade.set_id(14179278);
        trade.set_timestamp(1509576585);
        trade.set_order(create_order());
        trade
    }

    #[bench]
    fn serialize_order(b: &mut Bencher) {
        b.iter(|| create_order().write_to_bytes().unwrap());
    }

    #[bench]
    fn serialize_trade(b: &mut Bencher) {
        b.iter(|| create_trade().write_to_bytes().unwrap());
    }

    #[bench]
    fn deserialize_order(b: &mut Bencher) {
        let body = create_order().write_to_bytes().unwrap();
        b.iter(|| parse_from_bytes::<RawOrder>(&body).unwrap());
    }

    #[bench]
    fn deserialize_trade(b: &mut Bencher) {
        let body = create_trade().write_to_bytes().unwrap();
        b.iter(|| parse_from_bytes::<RawTrade>(&body).unwrap());
    }

    #[bench]
    fn serdeser_trade(b: &mut Bencher) {
        b.iter(|| {
            let body = create_trade().write_to_bytes().unwrap();
            let _ = parse_from_bytes::<RawTrade>(&body).unwrap();
        });
    }
}
