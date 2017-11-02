
// Notes:
//  * No heartbeats.
//  * No sequence IDs track.

use futures::sink::Sink;
use futures::stream::Stream;
use futures::future::{self, IntoFuture, Future, Loop};
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};
use core::Market;
use core::errors::*;
use core::reactor;
use streams::{Command, CommandReceiver, EventSender};
use utils::FutureExt;
use utils::ws::Client;

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

/// Connects to Poloniex WebSocket stream.
pub fn connect(
    sender: EventSender,
    commands: CommandReceiver,
    handle: &reactor::Handle,
) -> BoxFuture<()> {
    ClientBuilder::new(STREAM_URL)
        .unwrap()
        .async_connect_secure(None, handle)
        .and_then(move |(duplex, _)| {
            future::loop_fn(duplex, move |stream| {
                // check for core commands
                if let Ok(cmd) = commands.try_recv() {
                    return send_cmd(stream, cmd);
                }
                let sender = sender.clone();
                // continue reading from stream
                stream
                    .into_future()
                    .or_else(|(err, stream)| {
                        println!("Could not receive message: {:?}", err);
                        stream
                            .send(OwnedMessage::Close(None))
                            .map(|s| (None, s))
                            .into_box()
                    })
                    .and_then(move |(body, stream)| match body {
                        Some(OwnedMessage::Text(txt)) => {
                            match super::parser::parse_message(&txt) {
                                Ok(Some(msgs)) => {
                                    sender
                                        .unbounded_send((Market::Poloniex, msgs.events))
                                        .unwrap();
                                }
                                Ok(None) => {} // empty message
                                Err(err) => error!("poloniex websocket parse error: {:?}", err),
                            }
                            future::ok(Loop::Continue(stream)).into_box()
                        }
                        Some(OwnedMessage::Close(_)) => future::ok(Loop::Break(())).into_box(),
                        _ => {
                            warn!("unexpected message from poloniex websocket");
                            future::ok(Loop::Break(())).into_box()
                        }
                    })
                    .into_box()
            })
        })
        .map_err(|e| e.into())
        .into_box()
}

fn send_cmd<T>(
    stream: Client,
    cmd: Command,
) -> Box<Future<Item = Loop<T, Client>, Error = WebSocketError>> {
    stream
        .send(OwnedMessage::Text(super::cmd::serialize(cmd)))
        .map(|s| Loop::Continue(s))
        .into_box()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_cmd_fut_recv(b: &mut Bencher) {
        let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_fut_queue(10248);
        b.iter(|| match cmd_receiver.try_recv() {
            Ok(Command::Subscribe(pair)) => {
                println!("subscribe: {}", pair);
            }
            _ => {}
        });
    }

    #[bench]
    fn bench_cmd_recv(b: &mut Bencher) {
        let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_queue(10248);
        b.iter(|| match cmd_receiver.try_recv() {
            Ok(Command::Subscribe(pair)) => {
                println!("subscribe: {}", pair);
            }
            _ => {}
        });
    }
}
