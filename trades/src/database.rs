//! Trades database.

use std::fmt::Debug;
use std::path::Path;
use std::string::ToString;
use std::convert::Into;

use rayon::prelude::*;

use serde_json;

use lmdb::{EnvBuilder, Environment};
use lmdb::core::{DbHandle, DbIntKey, MdbResult};

use orca_currency::Pair;
use orca_markets::{Trade, RawTrade};
use super::SyncDatabase;


/// Trades database for a single market.
#[derive(Clone)]
pub struct Database {
    /// Database environment.
    pub env: Environment,
    /// Synchronization database.
    pub sync_db: SyncDatabase,
}

impl Database {
    /// Opens on-disk `lmdb` database and creates in-memory db.
    pub fn open<P: AsRef<Path> + Debug>(path: P) -> MdbResult<Self> {
        // create lmdb environment
        let env = EnvBuilder::new()
            .max_dbs(1000)
            .map_size(1e12 as u64)
            .open(&path, 0o777)?;
        // History sync info database
        let sync_db = SyncDatabase::open(&path)?;
        // return opened database
        debug!("Opened trades database {:?}", path);
        Ok(Database {
            env: env,
            sync_db: sync_db,
        })
    }

    /// Opens or creates currency pair database.
    pub fn open_db(&mut self, pair: &Pair) -> MdbResult<DbHandle> {
        self.env.create_db(&pair.to_string(), DbIntKey)
    }

    /// Writes trade to database.
    pub fn write(&mut self, trade: Trade) -> MdbResult<()> {
        // open database for single currency pair
        let db_handle = self.open_db(&trade.pair())?;
        // create a new transaction
        let txn = self.env.new_transaction()?;
        {
            // bind transaction to currency pair database
            let db = txn.bind(&db_handle);
            // serialize trade to JSON
            let trade_id = trade.id;
            let raw_trade: RawTrade = trade.into();
            let body = serde_json::to_string(&raw_trade).unwrap();
            // set JSON body under trade ID
            db.set(&trade_id, &body)?;
        }
        // commit transaction, returning it's result
        txn.commit()
    }

    /// Writes vector of trades to database.
    pub fn write_trades(&mut self, pair: &Pair, trades: Vec<RawTrade>) -> MdbResult<()> {
        // return on empty list
        if trades.len() == 0 {
            return Ok(())
        }
        // create vector of serialized trades
        let serialized_trades: Vec<(i64, String)> = trades.par_iter()
            .map(|ref trade| (trade.id, serde_json::to_string(trade).unwrap()))
            .collect();
        // open database for single currency pair
        let db_handle = self.open_db(pair)?;
        // create a new transaction
        let txn = self.env.new_transaction()?;
        {
            // bind transaction to currency pair database
            let db = txn.bind(&db_handle);
            // iterate over trades vector
            for &(ref id, ref body) in serialized_trades.iter() {
                db.set(id, body)?;
            }
        }
        // commit transaction
        txn.commit()
    }
}
