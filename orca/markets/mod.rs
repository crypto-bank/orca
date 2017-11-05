//! Orca markets module.

#[cfg(proto3)]
mod markets;
pub use self::markets::*;

use std::convert::TryFrom;

/// Tries to convert from integer to `OrderKind`.
/// Ask is `0` and Bid is `1.
impl TryFrom<i64> for OrderKind {
    type Error = ::core::Error;

    fn try_from(k: i64) -> Result<Self, Self::Error> {
        match k {
            0 => Ok(OrderKind::Ask),
            1 => Ok(OrderKind::Bid),
            _ => Err(::core::ErrorKind::InvalidOrderKind(k).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use protobuf::Message;
    use protobuf::parse_from_bytes;

    fn create_order() -> Order {
        let mut order = Order::new();
        order.set_kind(OrderKind::Bid);
        order.set_rate(0.00002854);
        order.set_volume(175.19271198);
        order
    }

    fn create_trade() -> Trade {
        let mut trade = Trade::new();
        trade.set_id(14179278);
        trade.set_timestamp(1509576585);
        trade.set_order(create_order());
        trade
    }

    #[bench]
    fn order_kind_try_from_i64(b: &mut Bencher) {
        b.iter(|| { OrderKind::try_from(0 as i64).unwrap(); });
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
        b.iter(|| parse_from_bytes::<Order>(&body).unwrap());
    }

    #[bench]
    fn deserialize_trade(b: &mut Bencher) {
        let body = create_trade().write_to_bytes().unwrap();
        b.iter(|| parse_from_bytes::<Trade>(&body).unwrap());
    }

    #[bench]
    fn serdeser_trade(b: &mut Bencher) {
        b.iter(|| {
            let body = create_trade().write_to_bytes().unwrap();
            let _ = parse_from_bytes::<Trade>(&body).unwrap();
        });
    }
}
