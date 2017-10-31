
use futures::future;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use websocket::async::TcpStream;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::client::async::{Client, TlsStream};
use websocket::async::futures::stream::{SplitSink, SplitStream};
use core::{reactor, BoxFuture, CurrencyPair};
// use core::errors::*;

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

pub struct PoloniexStream {
    sink: SplitSink<Client<TlsStream<TcpStream>>>,
}

impl ::core::streams::Stream for PoloniexStream {
    fn connect(handle: &reactor::Handle) -> BoxFuture<Self> {
        let connect = ClientBuilder::new(STREAM_URL)
            .unwrap()
            .async_connect_secure(None, handle)
            .and_then(|(duplex, _)| {
                let (sink, stream) = duplex.split();
                let stream_handler = stream
                    .filter_map(|message| {
                        println!("Received Message: {:?}", message);
                        None
                    })
                    // .select(close)
                    .forward(sink);
                    // .map_err(|e| e.into());
                handle.spawn(stream_handler);
                // Box::new(PoloniexStream { sink: sink })
                PoloniexStream { sink: sink }
            })
            .map_err(|e| e.into());
        Box::new(connect)
    }

    fn close(&mut self) -> BoxFuture<()> {
        Box::new(future::ok(()))
    }

    fn subscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()> {
        Box::new(future::ok(()))
    }

    fn unsubscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()> {
        Box::new(future::ok(()))
    }
}
