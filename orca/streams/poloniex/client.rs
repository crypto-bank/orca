
use streams::Command;
use utils::ws;

/// Poloniex WebSocket protocol implementation.
pub struct Client {
    handle: ws::Handle,
}

impl ::streams::Client for Client {
    fn handle(&self) -> ws::Handle {
        &self.handle
    }
}

impl ::streams::Protocol for Client {
    fn parse(msg: &str) -> ws::Message {
        super::parser::parse_message(msg)
    }

    fn serialize(cmd: &Command) -> String {
        super::cmd::serialize(cmd)
    }
}
