//! Poloniex WebSocket stream subscriber.

use std::convert::TryFrom;
use serde_json::Value;
use utils::try_opt;
use utils::parse::{get_array, get_object, get_i64, get_str, parse_str, parse_nth_str};
use core::errors::*;
use core::{OrderBook, OrderKind, RawOrder, RawTrade};
use streams::Event;
use utils::ws;

pub fn parse_message(text: &str) -> Result<Option<ws::Message>> {
    let msg = ::serde_json::from_str::<Value>(text)?;
    if try_opt(msg.as_array())?.len() <= 2 {
        return Ok(None);
    }
    let chan_id = get_i64(&msg, 0)?;
    let seq_id = get_i64(&msg, 1)?;
    let events = get_array(&msg, 2)?;
    let mut results = Vec::with_capacity(events.len());
    for event in events {
        results.push(parse_event(event)?);
    }
    Ok(Some(ws::Message {
        seq_id: seq_id,
        chan_id: chan_id,
        events: results,
    }))
}

fn parse_event(event: &Value) -> Result<Event> {
    match get_str(event, 0)? {
        "t" => Ok(Event::Trade(parse_trade(event)?)),
        "o" => Ok(Event::Order(parse_order(event)?)),
        "i" => Ok(Event::OrderBook(parse_order_book(try_opt(event.get(1))?)?)),
        any => Err(ErrorKind::UnexpectedEventType(any.to_owned()).into()),
    }
}

fn parse_order_book(event: &Value) -> Result<OrderBook> {
    let pair = get_str(event, "currencyPair")?;
    let pair = ::utils::parse_pair_reversed(pair)?;
    let books = get_array(event, "orderBook")?;
    let mut book = OrderBook::new(&pair);
    for (ref rate, ref volume) in get_object(books, 0)?.iter() {
        let rate = rate.as_str().parse::<f64>()?;
        let volume = parse_str::<f64>(volume)?;
        book.asks.set(rate, volume);
    }
    for (ref rate, ref volume) in get_object(books, 1)?.iter() {
        let rate = rate.as_str().parse::<f64>()?;
        let volume = parse_str::<f64>(volume)?;
        book.bids.set(rate, volume);
    }
    Ok(book)
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


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use protobuf::Message;
    use core::{OrderKind, RawOrder};
    use streams::Event;

    fn assert_eq_msg<M: Message>(a: &M, b: &M) {
        let ab = a.write_to_bytes().unwrap();
        let bb = b.write_to_bytes().unwrap();
        assert_eq!(ab, bb);
    }

    #[test]
    fn parse_test() {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"], \
                    [\"t\",\"14179278\",1,\"0.00002854\",\"175.19271198\",1509576585]]]";
        let msg = parse_message(body).unwrap().unwrap();
        assert_eq!(msg.chan_id, 117);
        assert_eq!(msg.seq_id, 103957441);
        assert_eq!(msg.events.len(), 2);
        if let &Event::Order(ref o) = msg.events.get(0).unwrap() {
            let mut order = RawOrder::new();
            order.set_kind(OrderKind::Bid);
            order.set_rate(0.00002789);
            order.set_volume(1788.27536750);
            assert_eq_msg(&order, o);
        } else {
            panic!("expecter order");
        }
        if let &Event::Trade(ref t) = msg.events.get(1).unwrap() {
            let mut order = RawOrder::new();
            order.set_kind(OrderKind::Bid);
            order.set_rate(0.00002854);
            order.set_volume(175.19271198);
            let mut trade = RawTrade::new();
            trade.set_id(14179278);
            trade.set_timestamp(1509576585);
            trade.set_order(order);
            assert_eq_msg(&trade, t);
        } else {
            panic!("expecter trade");
        }
    }

    #[bench]
    fn parse_body(b: &mut Bencher) {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"]]]";
        b.iter(|| { parse_message(body).unwrap(); });
    }

    #[bench]
    fn parse_body_two(b: &mut Bencher) {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"], \
                    [\"o\",1,\"0.00002784\",\"82074.71641065\"]]]";
        b.iter(|| { parse_message(body).unwrap(); });
    }
}
