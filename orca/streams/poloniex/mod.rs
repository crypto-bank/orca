//! Poloniex WebSocket stream.

mod cmd;
mod parser;

use ::core::errors::*;
use ::markets::Market;
use ::streams::Command;
use ::streams::ws::Message;

/// Poloniex WebSocket protocol.
pub struct Protocol;

/// #SPC-markets-streams-poloniex
impl ::streams::Protocol for Protocol {
    fn market() -> Market {
        Market::Poloniex
    }

    fn parse(msg: &str) -> Result<Option<Message>> {
        self::parser::parse_message(msg)
    }

    fn serialize(cmd: Command) -> String {
        self::cmd::serialize(cmd)
    }
}
