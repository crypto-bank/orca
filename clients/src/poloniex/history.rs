//! Poloniex trade history reader.

use std::string::ToString;

use serde_json;

use chrono::{self, UTC, TimeZone};

use futures::future::{self, Future};
use futures::stream::{self, Stream};

use hyper::{self, Client};
use hyper_tls::HttpsConnector;

use orca_currency::{Pair};
use orca_markets::{RawOrder, RawTrade, OrderKind};

/// Poloniex historical trade.
#[derive(Deserialize, Debug)]
struct HistoryTrade<'a> {
    #[serde(rename = "globalTradeID")]
    pub global_trade_id: i64,
    #[serde(rename = "tradeID")]
    pub trade_id: i64,
    pub date: &'a str,
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub rate: &'a str,
    pub amount: &'a str,
    pub total: &'a str,
}

impl<'a> HistoryTrade<'a> {
    /// Parses date and returns UNIX timestamp.
    fn timestamp(&self) -> Result<i64, chrono::format::ParseError> {
        let date = UTC.datetime_from_str(self.date, "%Y-%m-%d %H:%M:%S")?;
        Ok(date.timestamp())
    }

    /// Parses string to order kind/
    fn kind(&self) -> OrderKind {
        match self.kind {
            "sell" => OrderKind::Ask,
            "buy" => OrderKind::Bid,
            _ => unreachable!(),
        }
    }

    /// Creates a raw trade from historical trade.
    fn to_raw_trade(&self) -> RawTrade {
        RawTrade {
            id: self.trade_id,
            order: RawOrder {
                kind: self.kind(),
                rate: self.rate.parse::<f64>().unwrap(),
                volume: self.amount.parse::<f64>().unwrap(),
            },
            timestamp: self.timestamp().unwrap(),
        }
    }
}

/// Fetches trades history from start to end.
pub fn fetch_to_end(
        client: Client<HttpsConnector>,
        pair: Pair,
        start: i64,
        end: i64
    ) -> Box<Stream<Item=Box<Vec<RawTrade>>, Error=hyper::Error>>
{
    Box::new(stream::unfold(end, move |time_end| {
        // stop if time_end was set to -1
        if time_end == -1 {
            return None
        }
        // request trades from API
        let trades = fetch_trades(client.clone(), &pair, start, time_end);
        // map to trades and next end date
        Some(trades.map(|trades| {
            let next_end = if trades.len() >= 50000 {
                trades.last().unwrap().timestamp
            } else {
                -1 // if less than 50k it's last page
            };
            // return futures with trades and next `end` timestamp
            (trades, next_end)
        }))
    }))
}

/// Request historical trades from poloniex.
fn fetch_trades(
        client: Client<HttpsConnector>,
        pair: &Pair,
        start: i64,
        end: i64
    ) -> Box<Future<Item=Box<Vec<RawTrade>>, Error=hyper::Error>>
{
    // construct API url
    let url = format!(
        "https://poloniex.com/public?command=returnTradeHistory&currencyPair={}&start={}&end={}",
        pair.to_string(), start.to_string(), end.to_string());
    // parse stirng to hyper URI
    let url = url.parse::<hyper::Uri>().unwrap();
    // request historical trades from API
    trace!("Request {}", url);
    let get = client.get(url);
    // read response body to vector
    let read = get.and_then(|resp| {
        resp.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, ::hyper::Error>(v)
        })
    });
    // return boxed future
    // which is gonna deserialize body
    Box::new(read.map(|chunks| {
        // deserialize response
        let body = String::from_utf8(chunks).unwrap();
        let trades: Vec<HistoryTrade> = serde_json::from_str(&body).unwrap();
        Box::new(trades.iter().map(|t| t.to_raw_trade()).collect())
    }))
}

