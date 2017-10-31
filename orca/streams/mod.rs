//! Markets streams.

pub mod poloniex;

use core::errors::*;
use core::{reactor, Market};

/// Connects to a market stream with given core.
pub fn connect(handle: &reactor::Handle, market: &Market) -> Result<()> {
    match market {
        Poloniex => Ok(()),
        _ => {
            let name = <Market as ::protobuf::ProtobufEnum>::descriptor(market).name();
            Err(ErrorKind::InvalidMarket(name.to_owned()).into())
        }
    }
}
