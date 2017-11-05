//! Orca markets events.
// #SPC-markets-streams

mod cmd;
mod connect;
pub mod ws;
pub use self::cmd::*;
pub use self::connect::*;

// pub mod bitfinex;
pub mod poloniex;

use ::core::errors::*;
use ::currency::Pair;
use ::events::Event;
use ::markets::Market;

/// Market stream events.
pub type Events = (Market, Pair, Vec<Event>);

/// Market stream events sender.
pub type EventSender = ::futures::sync::mpsc::UnboundedSender<Events>;

/// Market stream events receiver.
pub type EventReceiver = ::futures::sync::mpsc::UnboundedReceiver<Events>;

/// WebSocket Stream protocol trait.
pub trait Protocol {
    /// Returns market ID.
    fn market() -> Market;

    /// Parses message.
    fn parse(msg: &str) -> Result<Option<ws::Message>>;

    /// Serializes command.
    fn serialize(cmd: Command) -> String;
}
