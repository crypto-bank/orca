//! Poloniex WebSocket stream.

mod cmd;
mod parser;

use core::Market;
use core::errors::*;
use streams::Command;
use util::ws;

/// Poloniex WebSocket protocol.
pub struct Protocol;

/// #SPC-streams-poloniex
impl ::streams::Protocol for Protocol {
    fn market() -> Market {
        Market::Poloniex
    }

    fn parse(msg: &str) -> Result<Option<ws::Message>> {
        self::parser::parse_message(msg)
    }

    fn serialize(cmd: Command) -> String {
        self::cmd::serialize(cmd)
    }
}
