
use futures::IntoFuture;
use futures::future;
use futures::future::Future;
use futures::future::Loop;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use websocket::async::TcpStream;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::client::async::{Client, TlsStream};
use websocket::async::futures::stream::{SplitSink, SplitStream};
use core::{reactor, BoxFuture, CurrencyPair};
use core::streams::{Event, Events};
use utils::boxfuture;
use utils::FutureExt;
use streams::poloniex::{cmd, parser};
use utils::parse_pair;
// use core::errors::*;

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

// fn handle_msg() {
//     println!("parsing: {}", txt);
//     let msg = parser::parse_message(txt)
//         .unwrap();
//     match msg {
//         Some(msg) => {
//             println!(
//                 "seq: {} chan: {} msgs: {}",
//                 msg.seq_id,
//                 msg.chan_id,
//                 msg.events.len()
//             );
//             for ev in msg.events {
//                 match ev {
//                     Event::Order(o) => {
//                         println!("order: {}@{}", o.volume, o.rate)
//                     }
//                     Event::Trade(t) => {
//                         println!(
//                             "trade: {}@{}",
//                             t.get_order().volume,
//                             t.get_order().rate
//                         )
//                     }
//                     Event::OrderBooks(_o) => {
//                         println!("orderbook ...");
//                     }
//                 }
//             }
//         }
//         None => println!("none..."),
//     }
// }

// match msg {
//     Some(OwnedMessage::Text(txt)) => {
//         boxfuture::ok(Loop::Continue(stream))
//     }
//     Some(OwnedMessage::Binary(bin)) => {
//         stream
//             .send(OwnedMessage::Binary(bin))
//             .map(|s| Loop::Continue(s))
//             .into_box()
//     }
//     Some(OwnedMessage::Ping(data)) => {
//         println!("PING: {:?}", data);
//         stream
//             .send(OwnedMessage::Pong(data))
//             .map(|s| Loop::Continue(s))
//             .into_box()
//     }
//     Some(OwnedMessage::Close(_)) => {
//         stream
//             .send(OwnedMessage::Close(None))
//             .map(|_| Loop::Break(()))
//             .into_box()
//     }
//     Some(OwnedMessage::Pong(_)) => boxfuture::ok(Loop::Continue(stream)),
//     None => boxfuture::ok(Loop::Break(())),
// }

pub struct Poloniex {
    handle: ::core::streams::Handle,
    client: Client<TlsStream<TcpStream>>,
}

impl ::core::streams::Connector for Poloniex {
    fn connect(handle: ::core::streams::Handle) -> BoxFuture<Box<Self>> {
        ClientBuilder::new(STREAM_URL)
            .unwrap()
            .async_connect_secure(None, handle.reactor())
            .map_err(move |e| e.into())
            .map(move |(duplex, _)| {
                println!("Connected");
                // polo.handle.reactor()
                //     .spawn(
                //         future::loop_fn(polo.client, |stream| {
                //         stream
                //             .into_future()
                //             .or_else(|(err, stream)| {
                //                 println!("Could not receive message: {:?}", err);
                //                 stream.send(OwnedMessage::Close(None))
                //                     .map(|s| (None, s))
                //                     .into_box()
                //             })
                //             .and_then(|(_msg, stream)| future::ok(Loop::Continue(stream)).into_box())
                //     }).or_else(|e| Ok(()) as Result<(), ()>));

                // Send first subscription message
                // Tests...
                Box::new(Poloniex { handle: handle, client: duplex })
            })
            .into_box()
    }

    fn close(&mut self) -> BoxFuture<()> {
        boxfuture::ok(())
    }

    fn subscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()> {
        // self.client.send(OwnedMessage::Text(cmd::subscribe(pair)))
        //     .map(move |_| ())
        //     .map_err(move |e| e.into())
        //     .into_box()
        boxfuture::ok(())
    }

    fn unsubscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()> {
        boxfuture::ok(())
    }
}
