
use core::CurrencyPair;
use utils::join_pair_reversed;

/// Creates a `subscribe` message for a given pair.
pub fn subscribe(pair: &CurrencyPair) -> String {
    format!("{{\"command\":\"subscribe\", \"channel\":\"{}\"}}", join_pair_reversed(pair, '_'))
}
