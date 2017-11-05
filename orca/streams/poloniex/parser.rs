// Poloniex WebSocket stream protocol parser.

use std::convert::TryFrom;
use serde_json::Value;
use protobuf::RepeatedField;

use ::core::errors::*;
use ::currency::{Pair, PairExt};
use ::markets::{OrderBook, OrderKind, Order, Trade};
use ::streams::Event;
use ::streams::ws;
use ::util::OptionExt;
use ::util::parse::*;


/// Parses Poloniex WebSocket message.
pub fn parse_message(text: &str) -> Result<Option<ws::Message>> {
    let msg = ::serde_json::from_str::<Value>(text)?;
    if msg.as_array().into_result()?.len() <= 2 {
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

/// Parses single event from Poloniex WebSocket message.
fn parse_event(event: &Value) -> Result<Event> {
    match get_str(event, 0)? {
        "t" => Ok(Event::Trade(parse_trade(event)?)),
        "o" => Ok(Event::Order(parse_order(event)?)),
        "i" => Ok(Event::OrderBook(
            parse_order_book(event.get(1).into_result()?)?,
        )),
        any => Err(ErrorKind::UnknownEventType(any.to_owned()).into()),
    }
}

/// Parses order book event.
fn parse_order_book(event: &Value) -> Result<OrderBook> {
    // Get books from event
    let books = get_array(event, "orderBook")?;
    let asks = get_object(books, 0)?;
    let bids = get_object(books, 1)?;

    // Create vector of orders
    let mut orders = Vec::with_capacity(asks.len() + bids.len());
    for (ref rate, ref volume) in asks.iter() {
        let mut order = Order::new();
        order.set_kind(OrderKind::Ask);
        order.set_rate(rate.as_str().parse::<f64>()?);
        order.set_volume(parse_str::<f64>(volume)?);
        orders.push(order);
    }
    for (ref rate, ref volume) in bids.iter() {
        let mut order = Order::new();
        order.set_kind(OrderKind::Bid);
        order.set_rate(rate.as_str().parse::<f64>()?);
        order.set_volume(parse_str::<f64>(volume)?);
        orders.push(order);
    }

    // Construct orderbook struct
    let mut book = OrderBook::new();
    let pair = get_str(event, "currencyPair")?;
    book.set_pair(Pair::parse_reversed(pair)?);
    book.set_orders(RepeatedField::from_vec(orders));

    Ok(book)
}

/// Parses order event.
fn parse_order(event: &Value) -> Result<Order> {
    let mut order = Order::new();
    let kind = get_i64(event, 1)?;
    order.set_kind(OrderKind::try_from(kind)?);
    order.set_rate(parse_nth_str::<f64>(event, 2)?);
    order.set_volume(parse_nth_str::<f64>(event, 3)?);
    Ok(order)
}

/// Parses trade event.
fn parse_trade(event: &Value) -> Result<Trade> {
    let mut trade = Trade::new();
    trade.set_id(parse_nth_str::<i64>(event, 1)?);
    let mut order = Order::new();
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
    use ::markets::{OrderKind, Order};
    use ::streams::Event;

    fn assert_eq_msg<M: Message>(a: &M, b: &M) {
        let ab = a.write_to_bytes().unwrap();
        let bb = b.write_to_bytes().unwrap();
        assert_eq!(ab, bb);
    }

    #[test]
    /// #TST-markets-streams-poloniex
    fn parse_test() {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"], \
                    [\"t\",\"14179278\",1,\"0.00002854\",\"175.19271198\",1509576585]]]";
        let msg = parse_message(body).unwrap().unwrap();
        assert_eq!(msg.chan_id, 117);
        assert_eq!(msg.seq_id, 103957441);
        assert_eq!(msg.events.len(), 2);
        if let &Event::Order(ref o) = msg.events.get(0).unwrap() {
            let mut order = Order::new();
            order.set_kind(OrderKind::Bid);
            order.set_rate(0.00002789);
            order.set_volume(1788.27536750);
            assert_eq_msg(&order, o);
        } else {
            panic!("expecter order");
        }
        if let &Event::Trade(ref t) = msg.events.get(1).unwrap() {
            let mut order = Order::new();
            order.set_kind(OrderKind::Bid);
            order.set_rate(0.00002854);
            order.set_volume(175.19271198);
            let mut trade = Trade::new();
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
    fn parse_body_log(b: &mut Bencher) {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"]]]";
        b.iter(|| {
            trace!("parsing msg: {}", &body);
            parse_message(body).unwrap();
        });
    }

    #[bench]
    fn parse_body_two(b: &mut Bencher) {
        let body = "[117,103957441,[[\"o\",1,\"0.00002789\",\"1788.27536750\"], \
                    [\"o\",1,\"0.00002784\",\"82074.71641065\"]]]";
        b.iter(|| { parse_message(body).unwrap(); });
    }
}
