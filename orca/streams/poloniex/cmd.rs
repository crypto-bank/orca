
use core::CurrencyPair;
use util::join_pair_reversed;
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
        "{{\"command\":\"subscribe\", \"channel\":\"{}\"}}",
        join_pair_reversed(pair, '_')
    )
}

/// Creates a `unsubscribe` message for a given pair.
fn unsubscribe(pair: &CurrencyPair) -> String {
    format!(
        "{{\"command\":\"unsubscribe\", \"channel\":\"{}\"}}",
        join_pair_reversed(pair, '_')
    )
}
