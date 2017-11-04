//! Bitfinex WebSocket stream.

mod cmd;
mod parser;

use core::Market;
use core::errors::*;
use streams::Command;
use util::ws;

/// Bitfinex WebSocket protocol.
pub struct Protocol;

impl ::streams::Protocol for Protocol {
    fn market() -> Market {
        Market::Bitfinex
    }

    fn parse(msg: &str) -> Result<Option<ws::Message>> {
        self::parser::parse_message(msg)
    }

    fn serialize(cmd: Command) -> String {
        self::cmd::serialize(cmd)
    }
}
