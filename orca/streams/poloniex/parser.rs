//! Poloniex WebSocket stream subscriber.

use std::convert::TryFrom;
use serde_json::Value;
use utils::try_opt;
use utils::parse::{get_array, get_object, get_i64, get_str, parse_str, parse_nth_str};
use core::errors::*;
use core::streams::{Event, Events};
use core::{OrderBooks, OrderKind, RawOrder, RawTrade};

/// Parsed Poloniex stream message.
pub struct Message {
    pub seq_id: i64,
    pub chan_id: i64,
    pub events: Events,
}

pub fn parse_message(text: String) -> Result<Option<Message>> {
    let msg = ::serde_json::from_str::<Value>(&text)?;
    if try_opt(msg.as_array())?.len() <= 2 {
        return Ok(None)
    }
    let chan_id = get_i64(&msg, 0)?;
    let seq_id = get_i64(&msg, 1)?;
    let events = get_array(&msg, 2)?;
    let mut results = Events::with_capacity(events.len());
    for event in events {
        results.push(parse_event(event)?);
    }
    Ok(Some(Message {
        seq_id: seq_id,
        chan_id: chan_id,
        events: results,
    }))
}

fn parse_event(event: &Value) -> Result<Event> {
    match get_str(event, 0)? {
        "t" => Ok(Event::Trade(parse_trade(event)?)),
        "o" => Ok(Event::Order(parse_order(event)?)),
        "i" => Ok(Event::OrderBooks(parse_order_book(try_opt(event.get(1))?)?)),
        any => Err(ErrorKind::UnexpectedEventType(any.to_owned()).into()),
    }
}

fn parse_order_book(event: &Value) -> Result<OrderBooks> {
    let pair = get_str(event, "currencyPair")?;
    let pair = ::utils::parse_pair_reversed(pair)?;
    let books = get_array(event, "orderBook")?;
    let mut order_books = OrderBooks::new(pair);
    for (ref rate, ref volume) in get_object(books, 0)?.iter() {
        let rate = rate.as_str().parse::<f64>()?;
        let volume = parse_str::<f64>(volume)?;
        order_books.asks.set(rate, volume);
    }
    for (ref rate, ref volume) in get_object(books, 1)?.iter() {
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
    order.set_rate(parse_nth_str::<f64>(event, 2)?);
    order.set_volume(parse_nth_str::<f64>(event, 3)?);
    Ok(order)
}

fn parse_trade(event: &Value) -> Result<RawTrade> {
    let mut trade = RawTrade::new();
    trade.set_id(parse_nth_str::<i64>(event, 1)?);
    let mut order = RawOrder::new();
    let kind = get_i64(event, 2)?;
    order.set_kind(OrderKind::try_from(kind)?);
    order.set_rate(parse_nth_str::<f64>(event, 3)?);
    order.set_volume(parse_nth_str::<f64>(event, 4)?);
    trade.set_order(order);
    trade.set_timestamp(get_i64(event, 5)?);
    Ok(trade)
}
