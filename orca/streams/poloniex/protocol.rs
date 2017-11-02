
use core::errors::*;
use streams::Command;
use utils::ws;

/// Poloniex WebSocket protocol implementation.
pub struct Protocol;

impl ::streams::Protocol for Protocol {
    fn parse(msg: &str) -> Result<Option<ws::Message>> {
        super::parser::parse_message(msg)
    }

    fn serialize(cmd: Command) -> String {
        super::cmd::serialize(cmd)
    }
}
