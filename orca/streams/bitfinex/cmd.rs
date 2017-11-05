
use ::core::currency::{Pair, PairExt};
use ::streams::Command;

/// Serializes a stream command.
pub fn serialize(cmd: Command) -> String {
    match cmd {
        Command::Subscribe(ref pair) => subscribe(pair),
        Command::Unsubscribe(ref pair) => unsubscribe(pair),
    }
}

/// Creates a `subscribe` message for a given pair.
fn subscribe(pair: &currency::Pair) -> String {
    format!(
        "{{\"event\":\"subscribe\",\"channel\":\"book\",\"pair\":\"{}\",\"prec\":\"R0\"}}",
        pair.join("")
    )
}

/// Creates a `unsubscribe` message for a given pair.
fn unsubscribe(_pair: &currency::Pair) -> String {
    unimplemented!()
}
