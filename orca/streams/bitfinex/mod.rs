//! Bitfinex WebSocket stream.

mod cmd;
mod parser;

use ::core::errors::*;
use ::markets::Market;
use ::streams::Command;
use ::streams::ws;

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
