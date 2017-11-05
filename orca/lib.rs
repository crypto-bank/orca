
#![feature(try_from)]
#![feature(rustc_private)]

#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;

#[cfg(feature = "with-bytes")]
extern crate bytes;

#[macro_use]
extern crate error_chain;

// extern crate num_traits;
extern crate ordered_float;

extern crate multiqueue;
extern crate websocket;
extern crate tokio_core;
extern crate futures;
extern crate protobuf;
extern crate serde_json;


pub mod core;
pub mod currency;
pub mod events;
pub mod markets;
pub mod streams;
pub mod util;
