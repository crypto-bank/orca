
use core::CurrencyPair;
use util::join_pair;
use streams::Command;

/// Serializes a stream command.
pub fn serialize(cmd: Command) -> String {
    match cmd {
        Command::Subscribe(ref pair) => subscribe(pair),
        Command::Unsubscribe(ref pair) => unsubscribe(pair),
    }
}

/// Creates a `subscribe` message for a given pair.
fn subscribe(pair: &CurrencyPair) -> String {
    format!(
        "{{\"event\":\"subscribe\",\"channel\":\"book\",\"pair\":\"{}\",\"prec\":\"R0\"}}",
        join_pair(pair, "")
    )
}

/// Creates a `unsubscribe` message for a given pair.
fn unsubscribe(_pair: &CurrencyPair) -> String {
    unimplemented!()
}
