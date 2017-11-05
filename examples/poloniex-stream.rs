//! Poloniex stream example.

#![feature(try_from)]

extern crate orca;
extern crate futures;
extern crate multiqueue;
extern crate env_logger;

use std::convert::TryFrom;

use futures::prelude::*;
use orca::currency::Pair;
use orca::events::Event;
use orca::streams::{ws, connect, poloniex, Command};
use orca::util::reactor;

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

/// #TST-markets-streams-ws
fn main() {
    env_logger::init().unwrap();

    let mut core = reactor::Core::new().unwrap();

    let (event_sender, event_receiver) = ::futures::sync::mpsc::unbounded();
    let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_fut_queue(10248);

    cmd_sender.try_send(Command::Subscribe(Pair::try_from("ETH_BTC").unwrap())).unwrap();
    cmd_sender.try_send(Command::Subscribe(Pair::try_from("BCH_BTC").unwrap())).unwrap();

    let handle = ws::Handle::new(event_sender, cmd_receiver, core.handle());
    let conn = connect::<poloniex::Protocol>(STREAM_URL, handle);

    let reader = event_receiver.for_each(move |(_m, _c, events)| {
        for e in events {
            match e {
                Event::Order(o) => println!("order {}/{}", o.rate, o.volume),
                Event::Trade(t) => {
                    println!("trade {}/{}", t.get_order().rate, t.get_order().volume)
                }
                Event::OrderBook(ref book) => println!("orderbook@{}", book.get_pair()),
            }
        }

        Ok(())
    });

    // spawn event reader
    core.handle().spawn(reader);

    // spawn connection
    core.run(conn).ok();
}
