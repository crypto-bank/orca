//! Markets streams.

use std::sync::Arc;
use futures::sync::mpsc;
use core::reactor;
use core::errors::*;
use core::{BoxFuture, CurrencyPair, OrderBooks, RawOrder, RawTrade};

/// Market stream events.
pub type Events = Vec<Event>;

/// Market stream event.
pub enum Event {
    /// Single trade.
    Trade(RawTrade),

    /// Single order book change.
    Order(RawOrder),

    /// Order books message.
    OrderBooks(OrderBooks),
}

/// Market stream connector.
pub trait Connector: Sized {
    /// Creates a connected stream..
    fn connect(handle: Handle) -> BoxFuture<Box<Self>>;

    /// Subscribes to currency pair stream.
    fn subscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()>;

    /// Unsubscribes from currency pair stream.
    fn unsubscribe(&mut self, pair: &CurrencyPair) -> BoxFuture<()>;

    /// Closes a stream.
    fn close(&mut self) -> BoxFuture<()>;
}

/// Market stream core.
pub struct Core {
    sender: Arc<mpsc::UnboundedSender<Events>>,
    _reader: Arc<mpsc::UnboundedReceiver<Events>>,
    handle: Arc<reactor::Handle>,
}

impl Core {
    /// Creates a new stream core.
    pub fn new(handle: reactor::Handle) -> Self {
        let (sender, reader) = mpsc::unbounded();
        Core {
            sender: Arc::new(sender),
            _reader: Arc::new(reader),
            handle: Arc::new(handle),
        }
    }

    /// Creates a core handle.
    pub fn handle(&self) -> Handle {
        Handle {
            sender: Arc::clone(&self.sender),
            handle: Arc::clone(&self.handle),
        }
    }
}

/// Market stream core handle.
pub struct Handle {
    sender: Arc<mpsc::UnboundedSender<Events>>,
    handle: Arc<reactor::Handle>,
}

impl Handle {
    /// Returns reference to stream sender.
    pub fn send(&self, events: Events) -> Result<()> {
        self.sender.unbounded_send(events)
            .map_err(|e| e.into())
    }

    /// Returns reference to reactor handle.
    pub fn reactor(&self) -> &reactor::Handle {
        self.handle.as_ref()
    }
}
