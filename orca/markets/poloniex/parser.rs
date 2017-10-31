//! Poloniex WebSocket stream subscriber.

use serde_json;
use std::str::FromStr;
use std::convert::{From, TryFrom};
use serde_json::Value;
use core::{Currency, CurrencyPair, OrderBooks, OrderKind, RawOrder, RawTrade};


fn parse_info(event: &Value) -> Result<OrderBooks> {
    let pair = try_opt(event.get("currencyPair"))?;
    let pair = try_opt(pair.as_str())?;
    let pair = parse_pair(pair)?;
    // self.channels.insert(chan_id, pair);
    // trace!("Subscribed to pair {}", pair);
    let books = try_opt(event.get("orderBook"))?;
    let books = try_opt(books.as_array())?;
    let mut order_books = OrderBooks::new(pair);
    for (ref rate, ref volume) in get_book(books, 0)?.iter() {
        let rate = rate.as_str().parse::<f64>()?;
        let volume = parse_str::<f64>(volume)?;
        order_books.asks.set(rate, volume);
    }
    for (ref rate, ref volume) in get_book(books, 1)?.iter() {
        let rate = rate.as_str().parse::<f64>()?;
        let volume = parse_str::<f64>(volume)?;
        order_books.bids.set(rate, volume);
    }
    Ok(order_books)
}

fn parse_order(event: &Value) -> Result<RawOrder> {
    let mut order = RawOrder::new();
    let kind = get_i64(event, 1)?;
    order.set_kind(OrderKind::try_from(kind)?);
    order.set_rate(event_parse::<f64>(event, 2)?);
    order.set_volume(event_parse::<f64>(event, 3)?);
    Ok(order)
}

fn parse_trade(event: &Value) -> Result<RawTrade> {
    let mut trade = RawTrade::new();
    trade.set_id(event_parse::<i64>(event, 1)?);
    let mut order = RawOrder::new();
    let kind = get_i64(event, 2)?;
    order.set_kind(OrderKind::try_from(kind)?);
    order.set_rate(event_parse::<f64>(event, 3)?);
    order.set_volume(event_parse::<f64>(event, 4)?);
    trade.set_order(order);
    trade.set_timestamp(get_i64(event, 5)?);
    Ok(trade)
}

fn parse_pair(s: &str) -> Result<CurrencyPair> {
    let v: Vec<&str> = s.split('_').collect();
    if v.len() != 2 {
        return Err(ErrorKind::CoreError(::core::errors::ErrorKind::InvalidCurrencyPair(s.to_string())).into());
    }
    let mut pair = CurrencyPair::new();
    pair.set_quote(Currency::try_from(v[1])?);
    pair.set_base(Currency::try_from(v[0])?);
    Ok(pair)
}

fn get_book<'a>(books: &'a Vec<Value>, index: usize) -> Result<&'a ::serde_json::Map<String, Value>> {
    match books.get(index) {
        Some(book) => match book.as_object() {
            Some(book) => Ok(book),
            None => Err(ErrorKind::NoValueError.into()),
        }
        None => Err(ErrorKind::NoValueError.into()),
    }
}

#[inline]
fn get_i64(event: &Value, index: usize) -> Result<i64> {
    match event.get(index) {
        Some(value) => {
            match value.as_i64() {
                Some(value) => Ok(value),
                None => Err(ErrorKind::NoValueError.into()),
            }
        },
        None => Err(ErrorKind::NoValueError.into()),
    }
}

#[inline]
fn event_parse<T: FromStr>(event: &Value, index: usize) -> Result<T>
where
    Error: From<<T as FromStr>::Err>,
{
    match event.get(index) {
        Some(value) => parse_str::<T>(value),
        None => Err(ErrorKind::NoValueError.into()),
    }
}

#[inline]
fn parse_str<T: FromStr>(value: &Value) -> Result<T>
where
    Error: From<<T as FromStr>::Err>,
{
    match value.as_str() {
        Some(value) => value.parse::<T>().map_err(|e| e.into()),
        None => Err(ErrorKind::NoValueError.into()),
    }
}

#[inline]
fn try_opt<T: Sized>(opt: Option<T>) -> Result<T> {
    match opt {
        Some(value) => Ok(value),
        None => Err(ErrorKind::NoValueError.into()),
    }
}

pub mod errors {
    error_chain! {
        links {
            CoreError(::core::errors::Error, ::core::errors::ErrorKind);
        }

        foreign_links {
            ParseIntError(::std::num::ParseIntError);
            ParseFloatError(::std::num::ParseFloatError);
            DeserializeError(::serde_json::error::Error);
        }

        errors {
            NoValueError {
                description("value not found in json")
            }
        }
    }
}

use self::errors::*;
