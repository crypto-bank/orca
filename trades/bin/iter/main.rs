
extern crate lmdb_rs as lmdb;
extern crate chrono;
extern crate orca_currency;
extern crate orca_markets;
extern crate orca_trades;
extern crate serde;
extern crate serde_json;
extern crate getopts;

extern crate ordered_float;

use std::env;
use std::str::FromStr;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use ordered_float::OrderedFloat;

use getopts::Options;

use orca_currency::Pair;
use orca_markets::{OrderMap, RawOrder, RawTrade, OrderKind};
use orca_trades::{Database, JsonIter};

fn print_usage(opts: Options) {
    println!("trades_export – exports trades to stdout in CSV or JSON format");
    println!("{}", opts.usage("Usage: trades_export [options]"));
}

fn main() {
    // create a default env path
    let default_path = env::home_dir()
        .map(|p| format!("{}/.cbank", p.display()))
        .unwrap_or_else(|| "/tmp/.cbank".to_owned());
    // contruct command line options
    let mut opts = Options::new();
    opts.optopt("d", "db", "db path", &default_path);
    opts.reqopt("p", "pair", "currency pair", "BTC_XRP");
    opts.optflag("s", "sum", "sum collect by timestamp");
    opts.optflag("t", "header", "write header");
    opts.optflag("h", "help", "print this help menu");
    
    // parse options from command line arguments
    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();

    // print help on --help flag
    if matches.opt_present("help") {
        print_usage(opts);
        return;
    }

    // currency pair from flag
    let pair = Pair::from_str(&matches.opt_str("p").expect("Currency pair in --pair or -p flag is required"))
        .expect("Currency pair is invalid");
    // db path from cmd option or default_path
    let db_path = matches.opt_str("d").unwrap_or(default_path.to_owned());
    // open pairs database
    let mut trades_db = Database::open(&db_path).expect("Unable to open trades database");
    // open pair database
    let db_handle = trades_db.open_db(&pair).unwrap();
    // get database reader
    let reader = trades_db.env.get_reader().unwrap();
    // bind to database handle
    let db = reader.bind(&db_handle);
    // write header if --header flag set
    if matches.opt_present("header") {
        println!("t,r,v");
    }
    
    for ref trade in CollectIter::new(&db, 60).unwrap() {
        println!("trade: {:?}", trade);
    }
}

#[derive(Debug)]
struct CollectResult {
    
}

struct CollectIter<'a> {
    inner: JsonIter<'a, RawTrade>,
    seconds: i64,
    current: Option<CollectResult>,
}

impl<'a> CollectIter<'a> {
    pub fn new(db: &'a lmdb::Database<'a>, seconds: i64) -> lmdb::core::MdbResult<CollectIter<'a>> {
        Ok(CollectIter {
            inner: JsonIter::<RawTrade>::new(&db)?,
            seconds: seconds,
            current: None,
        })
    }
}

impl<'a> Iterator for CollectIter<'a> {
    type Item = CollectResult;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // ...
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
