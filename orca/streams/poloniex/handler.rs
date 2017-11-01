
mod parser;

use parser::parse_message;
use websocket::{OwnedMessage};

// todo: move out
fn parse_ws_msg(body: OwnedMessage) -> Result<()> {
    let text = body.as_text()?;
    let msg = serde_json::from_str::<serde_json::Value>(text)?;
    parse_body(msg)
}
