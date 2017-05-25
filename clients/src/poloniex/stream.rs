//! Poloniex WebSocket stream subscriber.

use ws;
use url;
use serde_json;

use std::convert::From;
use std::str::FromStr;
use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;

use futures::sync::mpsc::UnboundedSender;

use orca_currency::{Pair, Volume};
use orca_markets::{Trade, Order, OrderBook, OrderKind, Event};


/// Subscribes to Poloniex WebSocket stream and writes events to sender.
pub fn connect(sender: UnboundedSender<Event>, pairs: Vec<Pair>) -> ws::Result<()> {
    let factory = StreamFactory { pairs: pairs, sender: sender };
    let mut ws = ws::WebSocket::new(factory).unwrap();
    let url = url::Url::parse("wss://api2.poloniex.com:443").unwrap();
    ws.connect(url)?;
    if let Err(e) = ws.run() {
        return Err(e)
    }
    // at some point we might to want return
    // ws::Result<ws::WebSocket<StreamFactory>>
    // so we can reconnect on close etc.
    // Ok(())
    Err(ws::Error::new(ws::ErrorKind::Internal, "Event loop closed"))
}

/// Poloniex WebSocket stream subscriber.
struct Stream {
    /// Stream output.
    out: ws::Sender,

    /// Stream writes to this view.
    sender: UnboundedSender<Event>,

    /// Mapping of channel ID to currency pair.
    channels: HashMap<i64, Pair>,

    /// Pairs we are going to subscribe to.
    pairs: Vec<Pair>,
}

impl Stream {
    /// Constructs new Stream.
    pub fn new(out: ws::Sender, pairs: Vec<Pair>, sender: UnboundedSender<Event>) -> Stream {
        Stream {
            out: out,
            sender: sender,
            channels: HashMap::new(),
            pairs: pairs,
        }
    }

    /// Handles order JSON message.
    fn on_info(&mut self, chan_id: i64, event: &serde_json::Value) {
        // get currency pair string
        let pair = event.get("currencyPair").unwrap().as_str().unwrap();
        let pair = Pair::from_str(pair).unwrap();

        trace!("Subscribed to pair {}", pair);

        // register pair on channel id
        self.channels.insert(chan_id, pair);

        // get order books
        let books = event.get("orderBook").unwrap().as_array().unwrap();

        // iterate over ask rates
        let mut asks = BTreeMap::new();
        for (ref rate, ref volume) in books.get(0).unwrap().as_object().unwrap().iter() {
            let rate = rate.as_str().parse::<f64>().unwrap();
            let volume = volume.as_str().unwrap().parse::<f64>().unwrap();
            asks.insert(OrderedFloat(rate), OrderedFloat(volume));
        }

        // iterate over bid rates
        let mut bids = BTreeMap::new();
        for (ref rate, ref volume) in books.get(1).unwrap().as_object().unwrap().iter() {
            let rate = rate.as_str().parse::<f64>().unwrap();
            let volume = volume.as_str().unwrap().parse::<f64>().unwrap();
            bids.insert(OrderedFloat(rate), OrderedFloat(volume));
        }

        // write order books
        self.sender.send(Event::OrderBook(OrderBook{
            pair: pair,
            asks: asks,
            bids: bids,
        })).unwrap();
    }

    /// Handles order JSON message.
    fn on_order(&mut self, chan_id: i64, event: &serde_json::Value) {
        let pair = self.channels.get(&chan_id).unwrap();
        let kind = OrderKind::from(event.get(1).unwrap().as_i64().unwrap());
        let rate = event.get(2).unwrap().as_str().unwrap().parse::<f64>().unwrap();
        let amount = event.get(3).unwrap().as_str().unwrap().parse::<f64>().unwrap();

        self.sender.send(Event::Order(Order {
            kind: kind,
            rate: Volume(pair.0, rate),
            volume: Volume(pair.1, amount),
        })).unwrap();
    }

    /// Handles trade JSON message.
    fn on_trade(&mut self, chan_id: i64, event: &serde_json::Value) {
        let pair = self.channels.get(&chan_id).unwrap();
        let order_id = event.get(1).unwrap().as_str().unwrap().parse::<i64>().unwrap();
        let kind = match event.get(2).unwrap().as_i64().unwrap() {
            0 => OrderKind::Ask,
            1 => OrderKind::Bid,
            _ => unreachable!(),
        };
        let rate = event.get(3).unwrap().as_str().unwrap().parse::<f64>().unwrap();
        let amount = event.get(4).unwrap().as_str().unwrap().parse::<f64>().unwrap();
        let timestamp = event.get(5).unwrap().as_i64().unwrap();

        self.sender.send(Event::Trade(Trade {
            id: order_id,
            order: Order {
                kind: kind,
                rate: Volume(pair.0, rate),
                volume: Volume(pair.1, amount),
            },
            timestamp: timestamp,
        })).unwrap();
    }
}

impl ws::Handler for Stream {
    /// Subscribes to orderbook updates and trades.
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        debug!("Subscribing to {} pairs", self.pairs.len());
        self.pairs.iter().map(|pair| {
            self.out.send(
                format!("{{\"command\":\"subscribe\", \"channel\":\"{}\"}}", pair)).unwrap()
        }).collect::<Vec<()>>();
        Ok(())
    }

    /// Handles WebSocket message.
    fn on_message(&mut self, body: ws::Message) -> ws::Result<()> {
        let msg = serde_json::from_str::<serde_json::Value>(body.as_text().unwrap()).unwrap();
        if msg.as_array().unwrap().len() <= 2 {
            return Ok(())
        }

        let chan_id = msg.get(0).unwrap().as_i64().unwrap();
        // let seq_id = msg.get(1).unwrap().as_i64().unwrap();

        let events = msg.get(2).unwrap().as_array().unwrap();
        for event in events {
            match event.get(0).unwrap().as_str().unwrap() {
                "i" => self.on_info(chan_id, event.get(1).unwrap()),
                "o" => self.on_order(chan_id, event),
                "t" => self.on_trade(chan_id, event),
                _   => println!("Unexpected event type from Poloniex."),
            }
        }

        Ok(())
    }

    /// Handles Stream
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        panic!("The client was close with code: {:?}, reason: {}", code, reason);
    }

    /// Handles error.
    fn on_error(&mut self, err: ws::Error) {
        panic!("The server encountered an error: {:?}", err);
    }
}

/// Poloniex stream factory.
struct StreamFactory {
    pairs: Vec<Pair>,
    sender: UnboundedSender<Event>,
}

impl ws::Factory for StreamFactory {
    type Handler = Stream;

    /// Called when connection is lost.
    ///
    /// TODO: It should reconnect.
    fn connection_lost(&mut self, _: Self::Handler) {
        panic!("connection lost");
    }

    /// Creates handler for a connection.
    fn connection_made(&mut self, out: ws::Sender) -> Self::Handler {
        Stream::new(out, self.pairs.clone(), self.sender.clone())
    }
}
