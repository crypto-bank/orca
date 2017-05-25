//! Market history sync info database.

use std::path::Path;

use lmdb::{DbFlags, EnvBuilder, Environment, FromMdbValue, ToMdbValue};
use lmdb::core::{MdbResult};

use orca_currency::Pair;

/// History sync database for a single market.
/// It is used when syncing trades history with market.
#[derive(Clone)]
pub struct SyncDatabase {
    db_env: Environment,
}

impl SyncDatabase {
    /// Opens on-disk `lmdb` database.
    pub fn open<P: AsRef<Path>>(path: P) -> MdbResult<Self> {
        // create lmdb environment
        let db_env = EnvBuilder::new()
            .max_dbs(1000)
            .map_size(1e12 as u64)
            .open(&path, 0o777)?;
        // return opened database
        Ok(SyncDatabase {
            db_env: db_env,
        })
    }

    /// Set database as synced.
    pub fn set_synced(&mut self, pair: &Pair) -> MdbResult<()> {
        let timestamp = self.get_latest_timestamp(pair)?;
        self.set_sync_info(pair, &"sn", &timestamp)
    }

    /// Get database synced timestamp.
    pub fn get_synced_timestamp(&mut self, pair: &Pair) -> MdbResult<i64> {
        self.get_sync_info::<i64>(pair, &"sn")
    }


    /// Set timestamp of latest historical trade written.
    pub fn set_latest_timestamp(&mut self, pair: &Pair, timestamp: &i64) -> MdbResult<()> {
        // get currently latest timestamp
        if let Ok(t) = self.get_latest_timestamp(pair) {
            // return if currently storing even more latest timestamp
            if *timestamp < t {
                return Ok(())
            }
        }
        self.set_sync_info(pair, &"lt", timestamp)
    }

    /// Get timestamp of latest historical trade written.
    pub fn get_latest_timestamp(&mut self, pair: &Pair) -> MdbResult<i64> {
        self.get_sync_info::<i64>(pair, &"lt")
    }


    /// Set timestamp of oldest historical trade written.
    pub fn set_oldest_timestamp(&mut self, pair: &Pair, timestamp: &i64) -> MdbResult<()> {
        // get currently oldest timestamp
        if let Ok(t) = self.get_oldest_timestamp(pair) {
            // return if currently storing even older timestamp
            if *timestamp > t {
                return Ok(())
            }
        }
        // set oldest timestamp
        self.set_sync_info(pair, &"ol", timestamp)
    }

    /// Get timestamp of oldest historical trade written.
    pub fn get_oldest_timestamp(&mut self, pair: &Pair) -> MdbResult<i64> {
        self.get_sync_info::<i64>(pair, &"ol")
    }


    /// Gets value from pair-sync database.
    fn get_sync_info<V: FromMdbValue>(&mut self, pair: &Pair, key: &ToMdbValue)
        -> MdbResult<V>
    {
        // open or create info db for pair
        let db_handle = self.db_env.create_db(&format!("{}-sync", pair), DbFlags::empty())?;
        // get read only transaction
        let reader = self.db_env.get_reader()?;
        // create database reader
        let db = reader.bind(&db_handle);
        // read value and return
        db.get::<V>(key)
    }

    /// Sets value in pair-sync database.
    fn set_sync_info(&mut self, pair: &Pair, key: &ToMdbValue, v: &ToMdbValue)
        -> MdbResult<()>
    {
        // open or create info db for pair
        let db_handle = self.db_env.create_db(&format!("{}-sync", pair), DbFlags::empty())?;
        // create a new transaction
        let txn = self.db_env.new_transaction()?;
        {
            // bind transaction to database handle
            let db = txn.bind(&db_handle);
            // set value under key
            db.set(key, v)?;
        }
        // commit transaction
        txn.commit()
    }
}
