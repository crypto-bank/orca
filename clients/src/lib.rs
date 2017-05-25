//! Markets API clients.

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

extern crate orca_markets;
extern crate orca_currency;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate ws;
extern crate url;

#[macro_use]
extern crate hyper;
extern crate hyper_tls;

extern crate tokio_core;
// extern crate tokio_io;

extern crate chrono;
extern crate futures;
extern crate ordered_float;

extern crate crypto;
extern crate data_encoding;

use futures::Future;

mod util;

pub mod errors;
pub mod poloniex;

/// Market client authorization data.
pub struct Auth {
    /// API Key.
    pub key: String,
    /// API Secret.
    pub secret: String,
}

/// Market API client response type.
pub type Response<T> = Future<Item = Box<T>, Error = errors::Error>;
