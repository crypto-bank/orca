
#![feature(try_from)]

#![feature(rustc_private)]
#[macro_use]
extern crate log;

#[cfg(feature = "with-bytes")]
extern crate bytes;

#[macro_use]
extern crate error_chain;

// extern crate num_traits;
extern crate ordered_float;

extern crate futures;
extern crate protobuf;
extern crate websocket;
extern crate serde_json;
extern crate tokio_core;

pub mod core;
pub mod markets;
pub mod utils;
