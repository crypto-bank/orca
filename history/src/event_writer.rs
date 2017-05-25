//! Market events writer.

use futures::{Future, Stream};
use futures::sync::mpsc::UnboundedReceiver;

use orca_markets::{Event, OrderBooks};
use orca_trades::Database;


/// Iterates over stream and writes events to database.
pub fn event_writer(mut books: OrderBooks, mut db: Database, rx: UnboundedReceiver<Event>)
    -> Box<Future<Item = (), Error = ()>>
{
    Box::new(rx.for_each(move |event| {
        match event {
            Event::Order(order) => {
                trace!("{:?}", order);
                books.update(order)
            },
            Event::OrderBook(book) => books.merge(book),
            Event::Trade(trade) => {
                debug!("{:?}", trade);
                db.write(trade).unwrap()
            },
            Event::Trades(pair, trades) => {
                debug!("Writing {} trades in pair {}", trades.len(), pair);
                db.write_trades(&pair, trades).unwrap()
            },
        }
        Ok::<(), ()>(())
    }))
}
