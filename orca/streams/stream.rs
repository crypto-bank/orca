
use futures::future::{self, Future};
use futures::prelude::*;
use websocket::OwnedMessage;
use core::{Market};
use core::errors::*;
use util::{ws, FutureExt};
use super::{Command, Event};

/// WebSocket Stream protocol trait.
pub trait Protocol {
    /// Returns market ID.
    fn market() -> Market;

    /// Parses message.
    fn parse(msg: &str) -> Result<Option<ws::Message>>;

    /// Serializes command.
    fn serialize(cmd: Command) -> String;
}

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
                    .or_else(|(err, stream)| close_with_err(stream, err))
                    .and_then(move |(body, stream)| match body {
                        Some(msg) => handle_message::<P>(msg, stream, handle),
                        None => ws::break_loop(), // closed stream
                    })
                    .into_box()
            })
        })
        .into_box()
}

fn handle_message<P: Protocol>(
    body: OwnedMessage,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    match body {
        OwnedMessage::Text(body) => handle_body::<P>(body, stream, handle),
        OwnedMessage::Close(_) => ws::break_loop(),
        _ => {
            // we will investigate further on any
            error!("unexpected websocket message");
            ws::break_loop()
        }
    }
}

fn handle_body<P: Protocol>(
    body: String,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    trace!("parsing msg: {}", &body);
    match P::parse(&body) {
        Ok(Some(msg)) => process_message::<P>(msg, stream, handle),
        Ok(None) => ws::continue_loop(stream, handle), // empty message
        Err(err) => {
            error!("websocket market {:?} parse error: {}", P::market(), err);
            ws::continue_loop(stream, handle)
        }
    }
}

fn process_message<P: Protocol>(
    msg: ws::Message,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    // handle registration
    if msg.events.len() == 1 {
        if let Event::OrderBook(_) = msg.events[0] {
            return handle_orderbook::<P>(msg, stream, handle);
        }
    }
    send_and_continue::<P>(msg, stream, handle)
}

fn handle_orderbook<P: Protocol>(
    msg: ws::Message,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    // unwrap orderbook from msg
    let pair = match msg.events[0] {
        Event::OrderBook(ref book) => book.pair.clone(),
        _ => panic!("code was mangled?"),
    };
    let handle = handle.with_register(msg.chan_id, pair);
    send_and_continue::<P>(msg, stream, handle)
}

fn send_and_continue<P: Protocol>(
    msg: ws::Message,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    let pair = match handle.pair(&msg.chan_id) {
        Some(pair) => pair,
        None => {
            error!(
                "message on unknown channel, market: {:?} chan: {:?}",
                P::market(),
                msg.chan_id
            );
            return ws::continue_loop(stream, handle);
        }
    };
    // send events
    handle
        .sender
        .unbounded_send((P::market(), pair, msg.events))
        .unwrap();
    // continue loop with registered pair
    ws::continue_loop(stream, handle)
}

fn send_cmd<P: Protocol>(cmd: Command, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    ws::send_msg(OwnedMessage::Text(P::serialize(cmd)), stream, handle)
}

fn close_with_err<E: ::std::error::Error>(
    stream: ws::Client,
    err: E,
) -> BoxFuture<(Option<OwnedMessage>, ws::Client)> {
    error!("Could not receive message: {:?}", err);
    stream
        .send(OwnedMessage::Close(None))
        .map(|stream| (None, stream))
        .map_err(|e| e.into())
        .into_box()
}
