
extern crate orca;
// extern crate tokio_core;
extern crate futures;
extern crate multiqueue;

use std::thread;

use futures::prelude::*;

use orca::core::reactor;
use orca::streams::{Command, Event};
// use orca::streams::Connector;
use orca::streams::poloniex;
use orca::utils::parse_pair;

fn main() {
    let mut core = reactor::Core::new().unwrap();
    // let streams = ::orca::streams::Core::new(core.handle());

    let (event_sender, event_receiver) = ::futures::sync::mpsc::unbounded();
    let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_fut_queue(10248);

    let pair = parse_pair("XRP_BTC").unwrap();

    // streams.command(Command::Subscribe(pair)).unwrap();
    cmd_sender.try_send(Command::Subscribe(pair)).unwrap();

    let conn = poloniex::connect(event_sender, cmd_receiver, &core.handle());

    let reader = event_receiver.for_each(|(m, events)| {
        // println!("event from market {:?}", m);

        for e in events {
            match e {
                Event::Order(o) => println!("order {}@{}", o.volume, o.rate),
                Event::Trade(t) => {
                    println!("order {}@{}", t.get_order().volume, t.get_order().rate)
                }
                Event::OrderBooks(_) => println!("orderbook"),
            }
        }

        Ok(())
    });

    core.handle().spawn(reader);

    core.run(conn).ok();
}
