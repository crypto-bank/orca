//! Market trades history sync utilities.

use chrono::{UTC, Duration};

use futures::stream::{self, Stream};
use futures::future::{self, Future};
use futures::sync::mpsc::UnboundedSender;

use hyper::{self, Client};
use hyper_tls::HttpsConnector;

use orca_currency::Pair;
use orca_markets::Event;
use orca_clients::poloniex;
use orca_trades::SyncDatabase;


/// Starts history syncing thread.
pub fn start_sync() {
    unimplemented!();
}

/// Syncs currency pairs history.
pub fn sync_pairs(
        tx: UnboundedSender<Event>,
        pairs: &Vec<Pair>,
        sync_db: SyncDatabase,
        client: Client<HttpsConnector>,
    ) -> Box<Future<Item=(), Error=hyper::Error>>
{
    // convert into a vector of results
    let pairs: Vec<Result<Pair, ()>> = pairs.iter().map(|pair| Ok(*pair)).collect();
    // reduce vector synchronizing all pairs
    // results are not collected, instead are sent to `tx`
    let result = stream::unfold(pairs, move |mut leftover| {
        match leftover.pop() {
            Some(pair) => {
                let tx = tx.clone();
                let pair = pair.unwrap();
                let sync_db = sync_db.clone();
                let client = client.clone();
                Some(sync_pair(tx, pair, sync_db, client)
                        .map(|_| ((), leftover)))
            },
            None => None
        }
    }).fold((), |v, _| future::ok::<_, hyper::Error>(v));
    // return boxed future
    Box::new(result)
}

/// Syncs currency pair history.
pub fn sync_pair(
        tx: UnboundedSender<Event>,
        pair: Pair,
        mut sync_db: SyncDatabase,
        client: Client<HttpsConnector>,
    ) -> Box<Future<Item=(), Error=hyper::Error>>
{
    // current date
    let time_now = UTC::now();
    // get start and end of history request
    let (start, end) = {
        match sync_db.get_synced_timestamp(&pair) {
            Ok(start) => (start, time_now.timestamp()),
            Err(_) => {
                let end = sync_db.get_oldest_timestamp(&pair).unwrap_or(time_now.timestamp());
                let start = (time_now - Duration::days(365)).timestamp();
                (start, end)
            },
        }
    };

    info!("Syncing currency pair {}", pair);
    let result = poloniex::history::fetch_to_end(client, pair, start, end)
        .filter(|trades| trades.len() > 0)
        .map(move |trades| {
            trace!("{} new trades, first trade ID: {} last ID: {}",
                trades.len(),
                trades.first().unwrap().id,
                trades.last().unwrap().id);

            {
                // write oldest and latest trade timestamps
                if let Some(t) = trades.last() {
                    let latest_timestamp = &trades.first().unwrap().timestamp;
                    sync_db.set_latest_timestamp(&pair, latest_timestamp).unwrap();
                    sync_db.set_oldest_timestamp(&pair, &t.timestamp).unwrap();
                }

                // if less than 50k - at this point we're synced
                if trades.len() < 50000 {
                    sync_db.set_synced(&pair).unwrap();
                }
            }

            // send trades to writer
            if trades.len() > 0 {
                tx.send(Event::Trades(pair, *trades)).unwrap();
            }
            ()
        })
        // fold to empty tuple (no output is expected)
        .fold((), |v, _| future::ok::<_, hyper::Error>(v))
        .map(move |_| {
            info!("Currency pair {} synced", pair);
            ()
        });

    // return boxed future
    Box::new(result)
}
