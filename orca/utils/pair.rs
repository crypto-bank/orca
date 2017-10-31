
use std::convert::TryFrom;
use utils::try_opt;
use core::errors::*;
use core::{Currency, CurrencyPair};

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
    let quote = try_opt(v.get(0))?;
    let base = try_opt(v.get(1))?;
    Ok((*quote, *base))
}
