
extern crate chrono;
extern crate orca_currency;
extern crate orca_markets;
extern crate orca_trades;
extern crate serde_json;
extern crate getopts;

use std::env;
use std::str::FromStr;

use getopts::Options;

use orca_currency::Pair;
use orca_markets::{RawTrade, OrderKind};
use orca_trades::Database;

fn print_usage(opts: Options) {
    println!("trades_export – exports trades to stdout in CSV or JSON format");
    println!("{}", opts.usage("Usage: trades_export [options]"));
}

fn main() {
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
    // db path from cmd option or "/tmp/.cbank"
    let db_path = matches.opt_str("d").unwrap_or("/tmp/.cbank".to_owned());
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
    let mut last_timestamp = 0;
    // current avg rate
    let mut ask_volume = 0.0;
    let mut bid_volume = 0.0;
    let mut current_rate = 0.0;
    let mut current_volume = 0.0;
    let mut current_price_volume = 0.0;
    // iterate over all items in db
    for item in db.iter().unwrap() {
        // unmarshal trade from JSON
        let value = item.get_value::<&str>();
        let trade = serde_json::from_str::<RawTrade>(value).unwrap();
        
        if !matches.opt_present("sum") {
            println!(
                "{},{},{},{}",
                trade.timestamp,
                trade.order.rate,
                trade.order.price(),
                (trade.order.kind as i64).to_string(),
            );
            continue
        }

        if last_timestamp == trade.timestamp || current_rate == trade.order.rate {
            current_price_volume = current_price_volume + trade.order.price();
            current_volume = current_volume + trade.order.volume;
            current_rate = current_price_volume / current_volume;
        } else if last_timestamp == 0 || last_timestamp < trade.timestamp {
            if last_timestamp < trade.timestamp {
                println!(
                    "{},{},{},{}",
                    last_timestamp,
                    current_rate,
                    ask_volume,
                    bid_volume,
                );
            }
            last_timestamp = trade.timestamp;
            current_rate = trade.order.rate;
            current_volume = trade.order.volume;
            current_price_volume = trade.order.price();
            match trade.order.kind {
                OrderKind::Ask => {
                    ask_volume += trade.order.price();
                    bid_volume = 0.0;
                },
                OrderKind::Bid => {
                    ask_volume = 0.0;
                    bid_volume += trade.order.price();
                },
            }
        };
        
        // write trade to stdout
    }

}
