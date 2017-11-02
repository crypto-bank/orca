
use std::cell::Cell;
use futures::future::{self, Future, Loop};
use futures::prelude::*;
use websocket::{OwnedMessage};
use core::{Market, CurrencyPair};
use core::errors::*;
use core::reactor;
use utils::{ws, boxfuture, FutureExt};
use super::{Command, CommandReceiver, Event, EventSender, Protocol};

/// Connects to WebSocket stream.
pub fn connect<P: Protocol>(addr: &str, handle: ws::Handle) -> BoxFuture<()> {
    ws::connect(addr, &handle.reactor)
        .and_then(move |stream| {
            future::loop_fn((stream, Some(handle)), move |(stream, handle)| {
                let handle = handle.unwrap();
                if let Ok(cmd) = handle.commands.try_recv() {
                    return send_cmd::<P>(cmd, stream, handle);
                }
                stream
                    .into_future()
                    .or_else(|(err, stream)| ws::close_with_err(stream, err))
                    .and_then(move |(body, stream)| match body {
                        Some(msg) => parse_body::<P>(msg, stream, handle),
                        None => ws::break_loop(), // closed stream
                    })
                    .into_box()
            })
        })
        .into_box()
}

fn parse_body<P: Protocol>(body: OwnedMessage, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    match body {
        OwnedMessage::Text(body) => parse_message::<P>(body, stream, handle),
        OwnedMessage::Close(_) => ws::break_loop(),
        _ => {
            // we will investigate further on any
            error!("unexpected poloniex message");
            ws::break_loop()
        }
    }
}

fn parse_message<P: Protocol>(body: String, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    trace!("parsing msg: {}", &body);
    match P::parse(&body) {
        Ok(Some(msg)) => handle_message(msg, stream, handle),
        Ok(None) => ws::continue_loop(stream, handle), // empty message
        Err(err) => {
            error!("poloniex websocket parse error: {:?}", err);
            ws::continue_loop(stream, handle)
        }
    }
}

fn handle_message(msg: ws::Message, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    // if msg.events.len() == 1 {
    //     if let Event::OrderBook(ref book) = msg.events[0] {
    //         // handle .with_chan(msg.chan_id, &book.pair)
    //         return boxfuture::ok()
    //     }
    // }
    // send events
    // handle.sender
    //     .unbounded_send((Market::Poloniex, pair, msg.events))
    //     .unwrap();
    ws::continue_loop(stream, handle)
}

fn send_cmd<P: Protocol>(cmd: Command, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    ws::send_msg(OwnedMessage::Text(P::serialize(cmd)), stream, handle)
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
