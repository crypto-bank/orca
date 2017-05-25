//! Trades history database.

#[macro_use]
extern crate log;

extern crate rayon;
extern crate serde;
extern crate serde_json;

extern crate orca_markets;
extern crate orca_currency;

extern crate lmdb_rs as lmdb;

mod database;
mod json_iter;
mod sync_db;

pub use self::database::Database;
pub use self::json_iter::JsonIter;
pub use self::sync_db::SyncDatabase;
