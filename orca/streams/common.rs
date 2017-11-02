
use core::errors::*;
use streams::Command;
use utils::ws::Message;

/// Stream protocol implementation.
pub trait Protocol {
    /// Parses message.
    fn parse(msg: &str) -> Result<Option<Message>>;

    /// Serializes command.
    fn serialize(cmd: Command) -> String;
}

// pub trait Handle {
//     fn send(&self, )
// }
