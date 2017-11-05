//! Market events storage.

use protobuf::RepeatedField;

use ::events::{Event, LogBody, LogEvent};
use ::markets::{Order, OrderBook, Trade};

/// Converts vector of events to log body.
impl From<Vec<Event>> for LogBody {
    /// Convert from vector of stream events.
    fn from(events: Vec<Event>) -> Self {
        let mut body = LogBody::new();
        let events = events.into_iter()
            .map(|e| e.into())
            .collect();
        body.set_events(RepeatedField::from_vec(events));
        body
    }
}

/// Converts event to log event.
impl From<Event> for LogEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Order(order) => order.into(),
            Event::Trade(trade) => trade.into(),
            Event::OrderBook(book) => book.into(),
        }
    }
}

/// Converts order to log event.
impl From<Order> for LogEvent {
    fn from(order: Order) -> Self {
        let mut event = LogEvent::new();
        event.set_order(order);
        event
    }
}

/// Converts order book to log event.
impl From<OrderBook> for LogEvent {
    fn from(book: OrderBook) -> Self {
        let mut event = LogEvent::new();
        event.set_book(book);
        event
    }
}

/// Converts trade to log event.
impl From<Trade> for LogEvent {
    fn from(trade: Trade) -> Self {
        let mut event = LogEvent::new();
        event.set_trade(trade);
        event
    }
}

// /// Converts stream event to message.
// /// TODO
// impl ::std::convert::TryFrom<LogEvent> for Event {
//     type Error = errors::Error;
// 
//     /// Tries to convert from message.
//     fn try_from(event: LogEvent) -> Result<Self, Self::Error> {
//         if event.has_order() {
//             return Ok(Event::Order(event.take_order()));
//         }
//         if event.has_trade() {
//             return Ok(Event::Order(event.take_trade()));
//         }
//         Err()
//     }
// }
