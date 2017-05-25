
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate futures;
extern crate serde_json;
extern crate tokio_core;
// extern crate tokio_io;
extern crate tokio_signal;
extern crate hyper;
extern crate hyper_tls;
extern crate getopts;

extern crate orca_markets;
extern crate orca_clients;
extern crate orca_command;
extern crate orca_currency;
extern crate orca_trades;
extern crate orca_history;

use std::env;
use std::thread;
use std::str::FromStr;
use std::time::Duration;

use futures::Stream;
use futures::sync::mpsc;

use tokio_core::reactor::{Core, Interval};

use hyper::Client;
use hyper_tls::HttpsConnector;

use getopts::Options;

use orca_markets as markets;
use orca_clients::poloniex;
use orca_command::Config;
use orca_currency::Pair;
use orca_trades::Database;
use orca_history as sync;

fn print_usage(opts: Options) {
    print!("{}", opts.usage("Usage: crypto_bank [options]"));
}

fn main() {
    // Setup logging
    pretty_env_logger::init().unwrap();

    // contruct command line options
    let mut opts = Options::new();
    opts.optopt("c", "config", "config file path", "config.toml");
    opts.optflag("h", "help", "print this help menu");
    
    // parse options from command line arguments
    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();

    // print help on --help flag
    if matches.opt_present("help") {
        print_usage(opts);
        return;
    }

    // config path from cmd option or "config.toml"
    let config_path = matches.opt_str("c")
        .unwrap_or("config.toml".to_owned());

    // Read config from file
    let config = Config::from_file(&config_path)
        .expect("Unable to parse config file");

    // Vector of pairs to subscribe
    let pairs = config.subscribe_pairs();

    // Open on-disk trades database
    let trades_db = Database::open(&config.database)
        .expect("Unable to open trades database");
    // Create in-memory market order books database
    let books_db = markets::OrderBooks::new();

    // Streams of events
    let (tx, rx) = mpsc::unbounded();

    // Connect to Poloniex stream
    // Subscribe to currency pairs
    let stream_thread = {
        let tx = tx.clone();
        let pairs = pairs.clone();
        thread::spawn(move || {
            info!("Connecting to Poloniex stream");
            poloniex::stream::connect(tx, pairs).unwrap();
        })
    };

    // Spawn events writer thread
    let event_writer = {
        let trades_db = trades_db.clone();
        let books_clone = books_db.clone();
        thread::spawn(move || {
            let mut core = Core::new().unwrap();
            core.run(sync::event_writer(books_clone, trades_db, rx)).unwrap();
        })
    };

    // spawn history syncing thread
    let tx = tx.clone();
    let sync_db = trades_db.sync_db.clone();
    let sync_thread = thread::spawn(move || {
        // Core event loop
        let mut core = Core::new().unwrap();

        // event loop handle
        let handle = core.handle();

        // HTTP client for history sync
        let client = Client::configure()
            .connector(HttpsConnector::new(1, &handle))
            .build(&handle);
        loop {
            let sync = sync::sync_pairs(tx.clone(), &pairs, sync_db.clone(), client.clone());
            core.run(sync).unwrap();
            // wait for a minute…
            thread::sleep(Duration::from_secs(60));
        }
    });

    // Core event loop
    let mut core = Core::new().unwrap();

    // Create a logging interval
    let interval = Interval::new(Duration::from_secs(1), &core.handle()).unwrap();

    // Start logging loop
    core.run(interval.for_each(move |_| {
        return Ok(());
        debug!("Hallo");
        let pair = Pair::from_str("BTC_XRP").unwrap();
        let books = books_db.books(&pair);
        // return if not-yet updated
        if books.is_none() {
            return Ok(());
        }
        // unwrap lock
        let books = books.unwrap();
        // lock and unwrap
        let books = books.read().unwrap();
        // print all orders
        for (&price, &amount) in &books.bids {
            trace!("price: {} amount: {}", price, amount);
        }
        Ok(())
    })).unwrap();

    // Wait for stream thread and sync thread to close
    stream_thread.join().unwrap();
    sync_thread.join().unwrap();
    event_writer.join().unwrap();
}
