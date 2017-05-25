//! Cryptocurrency markets and utilities.

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate ordered_float;

extern crate orca_currency;

mod events;
mod orderbooks;
mod orders;
mod trades;

pub use self::events::*;
pub use self::orderbooks::*;
pub use self::orders::*;
pub use self::trades::*;

/// Market identificator.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Market {
    /// Poloniex.com
    Poloniex,
    /// Bitfinex.com
    Bitfinex,
}
