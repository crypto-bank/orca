
// Notes:
//  * No heartbeats.
//  * No sequence IDs track.

use std::collections::HashMap;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::future::{self, IntoFuture, Future, Loop};
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};
use core::{Market, CurrencyPair};
use core::errors::*;
use core::reactor;
use streams::{Command, CommandReceiver, EventSender};
use utils::{ws, FutureExt};

const STREAM_URL: &'static str = "wss://api2.poloniex.com:443";

/// Connects to Poloniex WebSocket stream.
pub fn connect(
    sender: EventSender,
    commands: CommandReceiver,
    handle: &reactor::Handle,
) -> BoxFuture<()> {
    ws::connect(STREAM_URL, handle)
        .and_then(move |stream| {
            let conn = Conn::new(sender, commands);
            future::loop_fn((stream, Some(conn)), move |(stream, conn)| {
                let conn = conn.unwrap();
                if let Ok(cmd) = conn.commands.try_recv() {
                    return send_cmd(cmd, stream, conn);
                }
                stream
                    .into_future()
                    .or_else(|(err, stream)| close_stream_err(stream, err))
                    .and_then(move |(body, stream)| parse_msg(body, stream, conn))
                    .into_box()
            })
        })
        .into_box()
}

type Transfer = (ws::Client, Option<Conn>);

fn parse_msg(
    body: Option<OwnedMessage>,
    stream: ws::Client,
    conn: Conn,
) -> BoxFuture<Loop<(), Transfer>> {
    match body {
        Some(OwnedMessage::Text(txt)) => {
            match super::parser::parse_message(&txt) {
                Ok(Some(msgs)) => {
                    conn.sender
                        .unbounded_send((Market::Poloniex, msgs.events))
                        .unwrap();
                }
                Ok(None) => {} // empty message
                Err(err) => error!("poloniex websocket parse error: {:?}", err),
            }
            continue_stream(stream, conn)
        }
        Some(OwnedMessage::Close(_)) => break_stream(),
        None => break_stream(),
        _ => {
            // we will investigate further on any
            error!("unexpected poloniex message");
            break_stream()
        }
    }
}

#[inline]
fn break_stream() -> BoxFuture<Loop<(), Transfer>> {
    future::ok(Loop::Break(())).into_box()
}

#[inline]
fn continue_stream(stream: ws::Client, conn: Conn) -> BoxFuture<Loop<(), Transfer>> {
    future::ok(Loop::Continue((stream, Some(conn)))).into_box()
}

#[inline]
fn send_cmd(cmd: Command, stream: ws::Client, conn: Conn) -> BoxFuture<Loop<(), Transfer>> {
    stream
        .send(OwnedMessage::Text(super::cmd::serialize(cmd)))
        .map(|stream| Loop::Continue((stream, Some(conn))))
        .map_err(|e| e.into())
        .into_box()
}

#[inline]
fn close_stream_err(
    stream: ws::Client,
    err: WebSocketError,
) -> BoxFuture<(Option<OwnedMessage>, ws::Client)> {
    error!("Could not receive message: {:?}", err);
    stream
        .send(OwnedMessage::Close(None))
        .map(|stream| (None, stream))
        .map_err(|e| e.into())
        .into_box()
}

struct Conn {
    sender: EventSender,
    commands: CommandReceiver,
    // channel id to currency pair
    pairs: HashMap<i64, CurrencyPair>,
}

impl Conn {
    /// Creates new connection struct.
    fn new(sender: EventSender, commands: CommandReceiver) -> Self {
        Conn {
            sender: sender,
            commands: commands,
            pairs: HashMap::new(),
        }
    }

    /// Returns currency pair by registered channel id.
    fn chan(&self, id: &i64) -> Option<&::core::CurrencyPair> {
        self.pairs.get(id)
    }

    /// Registers currency pair under channel id.
    fn set_chan(&mut self, id: i64, pair: ::core::CurrencyPair) {
        self.pairs.insert(id, pair);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn cmd_fut_empty_recv(b: &mut Bencher) {
        let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_fut_queue(10248);
        b.iter(|| match cmd_receiver.try_recv() {
            Ok(Command::Subscribe(pair)) => {
                println!("subscribe: {}", pair);
            }
            _ => {}
        });
    }

    #[bench]
    fn cmd_empty_recv(b: &mut Bencher) {
        let (cmd_sender, cmd_receiver) = ::multiqueue::broadcast_queue(10248);
        b.iter(|| match cmd_receiver.try_recv() {
            Ok(Command::Subscribe(pair)) => {
                println!("subscribe: {}", pair);
            }
            _ => {}
        });
    }
}
