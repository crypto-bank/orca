
#[cfg(proto3)]
pub mod currency;
#[cfg(proto3)]
pub mod order;
#[cfg(proto3)]
pub mod trade;
pub mod streams;
mod orderbook;

pub use self::currency::*;
pub use self::order::*;
pub use self::trade::*;
pub use self::orderbook::*;

// helpful conversion utilities

pub mod errors {
    error_chain! {
        errors {
            InvalidCurrencyPair(pair: String) {
                description("invalid currency pair")
                display("invalid currency pair: {}", pair)
            }
            InvalidOrderKind(k: i64) {
                description("invalid order kind")
                display("invalid order kind: {}", k)
            }
        }
    }
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
