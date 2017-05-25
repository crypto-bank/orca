
extern crate pretty_env_logger;
extern crate getopts;

extern crate futures;
extern crate tokio_core;
extern crate hyper;
extern crate hyper_tls;
extern crate orca;
extern crate orca_clients;

use std::env;

use futures::Future;

use getopts::Options;

use orca_clients::{Auth};
use orca_clients::poloniex;

fn print_usage(opts: Options) {
    print!("{}", opts.usage("Usage: crypto_bank [options]"));
}

fn main() {
    // Setup logging
    pretty_env_logger::init().unwrap();

    // contruct command line options
    let mut opts = Options::new();
    opts.reqopt("k", "key", "api key (required)", "");
    opts.reqopt("s", "secret", "api secret (required)", "");
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

    let mut core = ::tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let http = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(1, &handle))
        .build(&handle);
    let auth = Auth {
        key: "".to_owned(),
        secret: "".to_owned(),
    };
    let mut client = poloniex::Client::new(http, auth);

    // let printb = client.balances().then(|res| {
    //     println!("res: {:?}", res.unwrap());
    // });

    core.run(client.balances().map(|res| {
        println!("res: {:?}", res);
        ()
    })).unwrap();
}
