// Currency pairs utilities.

use std::fmt;
use std::fmt::{Formatter, Display};
use std::convert::TryFrom;

use ::core::errors::*;
use ::currency::{Symbol, Pair};
use ::util::OptionExt;

/// Pair extension.
pub trait PairExt {
    /// Joins pair with given separator.
    fn join<S: Display>(&self, sep: S) -> String;

    /// Joins reversed pair with given separator.
    fn join_reversed<S: Display>(&self, sep: S) -> String;

    /// Parses a currency pair with `_` separator and reversed.
    /// Quote and base are reversed in Poloniex APIs
    /// If you are looking for `parse` method see `TryFrom`.
    fn parse_reversed(s: &str) -> Result<Pair>;
}

/// Pair extension implementatino for Pair.
impl PairExt for Pair {
    /// Joins pair with given separator.
    fn join<S: Display>(&self, sep: S) -> String {
        join_pair(self, sep)
    }

    /// Joins reversed pair with given separator.
    fn join_reversed<S: Display>(&self, sep: S) -> String {
        join_pair_reversed(self, sep)
    }

    /// Parses a currency pair with `_` separator and reversed.
    /// Note: Quote and base are reversed in Poloniex APIs
    fn parse_reversed(s: &str) -> Result<Pair> {
        parse_pair_reversed(s)
    }
}

/// Formats currency pair for display.
/// TODO: convert this to `Debug`
impl Display for Pair {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}_{:?}", self.quote, self.base)
    }
}

/// Tries to convert currency pair from string.
impl<'a> TryFrom<&'a str> for Pair {
    type Error = Error;

    fn try_from(pair: &str) -> ::std::result::Result<Self, Self::Error> {
        parse_pair(pair)
    }
}

/// Joins pair with given separator.
fn join_pair<S: ::std::fmt::Display>(pair: &Pair, sep: S) -> String {
    format!("{:?}{}{:?}", pair.quote, sep, pair.base)
}

/// Joins pair with given separator.
fn join_pair_reversed<S: ::std::fmt::Display>(pair: &Pair, sep: S) -> String {
    format!("{:?}{}{:?}", pair.base, sep, pair.quote)
}

/// Parses a currency pair with `_` separator.
fn parse_pair(s: &str) -> Result<Pair> {
    let (quote, base) = parse_quote_base(s)?;
    let mut pair = Pair::new();
    pair.set_quote(Symbol::try_from(quote)?);
    pair.set_base(Symbol::try_from(base)?);
    Ok(pair)
}

/// Parses a currency pair with `_` separator and reversed.
/// Note: Quote and base are reversed in Poloniex APIs
fn parse_pair_reversed(s: &str) -> Result<Pair> {
    let (base, quote) = parse_quote_base(s)?;
    let mut pair = Pair::new();
    pair.set_quote(Symbol::try_from(quote)?);
    pair.set_base(Symbol::try_from(base)?);
    Ok(pair)
}

fn parse_quote_base(s: &str) -> Result<(&str, &str)> {
    let v = s.split('_').collect::<Vec<&str>>();
    if v.len() != 2 {
        return Err(ErrorKind::InvalidPair(s.to_string()).into());
    }
    let quote = v.get(0).into_result()?;
    let base = v.get(1).into_result()?;
    Ok((*quote, *base))
}

#[test]
fn parse_and_join() {
    let pair = parse_pair("XRP_BTC").unwrap();
    let revp = parse_pair_reversed("BTC_XRP").unwrap();
    assert_eq!(pair, revp);
    assert_eq!(join_pair(&pair, '_'), "XRP_BTC");
    assert_eq!(join_pair(&pair, ""), "XRPBTC");
    assert_eq!(join_pair_reversed(&pair, '_'), "BTC_XRP");
}
