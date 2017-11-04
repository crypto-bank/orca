
use std::convert::TryFrom;
use core::errors::*;
use core::{Currency, CurrencyPair};
use super::OptionExt;

/// Joins pair with given separator.
pub fn join_pair<S: ::std::fmt::Display>(pair: &CurrencyPair, sep: S) -> String {
    format!("{:?}{}{:?}", pair.quote, sep, pair.base)
}

/// Joins pair with given separator.
pub fn join_pair_reversed<S: ::std::fmt::Display>(pair: &CurrencyPair, sep: S) -> String {
    format!("{:?}{}{:?}", pair.base, sep, pair.quote)
}

/// Parses a currency pair with `_` separator.
pub fn parse_pair(s: &str) -> Result<CurrencyPair> {
    let (quote, base) = parse_quote_base(s)?;
    let mut pair = CurrencyPair::new();
    pair.set_quote(Currency::try_from(quote)?);
    pair.set_base(Currency::try_from(base)?);
    Ok(pair)
}

/// Parses a currency pair with `_` separator and reversed.
/// Note: Quote and base are reversed in Poloniex APIs
pub fn parse_pair_reversed(s: &str) -> Result<CurrencyPair> {
    let (base, quote) = parse_quote_base(s)?;
    let mut pair = CurrencyPair::new();
    pair.set_quote(Currency::try_from(quote)?);
    pair.set_base(Currency::try_from(base)?);
    Ok(pair)
}

fn parse_quote_base(s: &str) -> Result<(&str, &str)> {
    let v = s.split('_').collect::<Vec<&str>>();
    if v.len() != 2 {
        return Err(ErrorKind::InvalidCurrencyPair(s.to_string()).into());
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
