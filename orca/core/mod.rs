
#[cfg(proto3)]
pub mod currency;
#[cfg(proto3)]
pub mod market;
#[cfg(proto3)]
pub mod order;
#[cfg(proto3)]
pub mod trade;
pub mod streams;
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
            0 => Ok(OrderKind::ASK),
            1 => Ok(OrderKind::BID),
            _ => Err(errors::ErrorKind::InvalidOrderKind(k).into()),
        }
    }
}

/// NOTE: this operation is pretty expensive
/// TODO: it should use generated lookup table
impl<'a> ::std::convert::TryFrom<&'a str> for Currency {
    type Error = errors::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        for c in <Currency as ::protobuf::ProtobufEnum>::values() {
            if <Currency as ::protobuf::ProtobufEnum>::descriptor(c).name() == name {
                return Ok(c.clone());
            }
        }
        Err(errors::ErrorKind::InvalidCurrencyPair(name.to_owned()).into())
    }
}

impl ::std::fmt::Display for Currency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(<Currency as ::protobuf::ProtobufEnum>::descriptor(self).name())
    }
}

#[test]
fn display_currency() {
    assert_eq!(format!("{}", Currency::BTC), "BTC");
}
