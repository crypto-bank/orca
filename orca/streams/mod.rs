//! Markets streams.

pub mod poloniex;

use core::errors::*;
use core::{reactor, Market};

/// Connects to a market stream with given core.
pub fn connect(market: Market, handle: &reactor::Handle) -> Result<()> {
    match market {
        Market::Poloniex => Ok(()),
        _ => {
            let name = <Market as ::protobuf::ProtobufEnum>::descriptor(&market).name();
            Err(ErrorKind::InvalidMarket(name.to_owned()).into())
        }
    }
}
