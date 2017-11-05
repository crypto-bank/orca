// WebSocket stream connection.

use futures::future::{self, Future};
use futures::prelude::*;
use websocket::{OwnedMessage, WebSocketError};

use ::core::errors::*;
use ::events::{Event, EventExt};
use ::streams::{Protocol, Command};
use ::streams::ws;
use ::util::FutureExt;

/// Connects to WebSocket stream.
///
/// #SPC-markets-streams-ws
pub fn connect<P: Protocol>(addr: &str, handle: ws::Handle) -> BoxFuture<()> {
    fn close_with_err(
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

    // Connect to WebSocket server
    ws::connect(addr, &handle.reactor)
        .and_then(move |stream| {
            // Start connection stream processing loop.
            future::loop_fn((stream, Some(handle)), move |(stream, handle)| {
                // Check for subscribe commands.
                let handle = handle.unwrap();
                if let Ok(cmd) = handle.commands.try_recv() {
                    // Serialize and send commands to connection.
                    return send_cmd::<P>(cmd, stream, handle);
                }
                // Read from stream and process messages.
                // Break loop if connection was closed.
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
        .map_err(|e| e.into())
        .into_box()
}

/// Parse raw websocket message.
/// It can be `Close`, `Text` or a wrong message.
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

/// Parses body of a `Text` WebSocket message.
/// Processes message and sends events if any.
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

/// Processes message and sends events if any.
/// Registers currency pair channel on orderbook.
fn process_message<P: Protocol>(
    msg: ws::Message,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    // Check if event is order book
    if msg.events.is_order_book() {
        // Register new channel and send events
        return handle_orderbook::<P>(msg, stream, handle);
    }
    send_and_continue::<P>(msg, stream, handle)
}

/// Registers currency pair channel id in `ws::Handle`.
/// Sends parsed events to handle sender.
fn handle_orderbook<P: Protocol>(
    msg: ws::Message,
    stream: ws::Client,
    handle: ws::Handle,
) -> ws::LoopFuture {
    // unwrap orderbook from msg
    let pair = match msg.events[0] {
        Event::OrderBook(ref book) => book.get_pair().clone(),
        _ => panic!("code was mangled?"),
    };
    let handle = handle.with_channel(msg.chan_id, pair);
    send_and_continue::<P>(msg, stream, handle)
}

/// Sends message to handle sender.
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

/// Serializes an sends command to WebSocket connection.
fn send_cmd<P: Protocol>(cmd: Command, stream: ws::Client, handle: ws::Handle) -> ws::LoopFuture {
    ws::send_msg(OwnedMessage::Text(P::serialize(cmd)), stream, handle)
}
