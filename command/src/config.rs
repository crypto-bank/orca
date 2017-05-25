//! Configuration file reading.

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use toml;
use toml::de::Error;

use orca_currency::Pair;

/// Application configuration.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Database path.
    pub database: String,

    /// Currency pairs.
    pub pairs: Vec<String>,
}

impl Config {
    /// Reads config from file.
    /// It unwraps unsafely.
    pub fn from_file(path: &str) -> Result<Config, Error> {
        // open config file
        let mut file = File::open(path)
                .expect("Unable to open config file");

        // contents of a config file
        let mut contents = String::new();

        // read file to string
        file.read_to_string(&mut contents)
                .expect("Unable to read config file");

        // parse toml string to config
        toml::from_str(&contents)
    }

    /// Currency pairs.
    pub fn subscribe_pairs(&self) -> Vec<Pair> {
        self.pairs.iter().map(|p| Pair::from_str(&p).unwrap()).collect()
    }
}
