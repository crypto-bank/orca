
mod cmd;
mod parser;

use core::errors::*;
use streams::Command;
use util::ws;

/// Poloniex WebSocket protocol implementation.
pub struct Protocol;

impl ::streams::Protocol for Protocol {
    fn parse(msg: &str) -> Result<Option<ws::Message>> {
        self::parser::parse_message(msg)
    }

    fn serialize(cmd: Command) -> String {
        self::cmd::serialize(cmd)
    }
}
