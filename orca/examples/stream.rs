
#![feature(rustc_private)]
#[macro_use]
extern crate log;

extern crate orca;
// extern crate tokio_core;
extern crate futures;
extern crate multiqueue;
extern crate env_logger;

use futures::prelude::*;
use orca::core::reactor;
use orca::streams::{connect, Command, Event};
use orca::streams::poloniex;
use orca::utils::{parse_pair, ws};

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

fn main() {
    env_logger::init().unwrap();

    info!("hello");
    println!("hello");

    let mut core = reactor::Core::new().unwrap();
    // let streams = ::orca::streams::Core::new(core.handle());

    let (event_sender, event_receiver) = ::futures::sync::mpsc::unbounded();
    let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_fut_queue(10248);

    let pair = parse_pair("XRP_BTC").unwrap();

    info!("subscribing {}", pair);

    // streams.command(Command::Subscribe(pair)).unwrap();
    cmd_sender.try_send(Command::Subscribe(pair)).unwrap();

    info!("subscribed");

    let handle = ws::Handle::new(event_sender, cmd_receiver, core.handle());
    let conn = connect::<poloniex::Protocol>(STREAM_URL, handle);

    let reader = event_receiver.for_each(|(_m, _c, events)| {
        for e in events {
            match e {
                Event::Order(o) => println!("order {}@{}", o.volume, o.rate),
                Event::Trade(t) => {
                    println!("order {}@{}", t.get_order().volume, t.get_order().rate)
                }
                Event::OrderBook(ref book) => println!("orderbook@{}", book.pair),
            }
        }

        Ok(())
    });

    core.handle().spawn(reader);

    info!("spawned conn");

    core.run(conn).ok();
}
