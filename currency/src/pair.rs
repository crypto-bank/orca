//! Cryptocurrency pair.

use std::fmt;
use std::str::FromStr;

use symbol::Symbol;

/// Pair of currency symbols
/// Used to identify markets eq. `BTC/USD`
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Pair(pub Symbol, pub Symbol);

/// Displays currency pair in format eq. "BTC_USD"
impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}_{:?}", self.0, self.1)
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('_').collect();
        if split.len() != 2 {
            return Err("invalid currency pair".to_owned())
        }
        let first = try!(Symbol::from_str(split[0]));
        let second = try!(Symbol::from_str(split[1]));
        Ok(Pair(first, second))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn pair_from_str() {
        let pair = Pair::from_str("BTC_USD").unwrap();
        assert_eq!(pair.0, Symbol::BTC);
        assert_eq!(pair.1, Symbol::USD);
    }

    #[test]
    #[should_panic]
    fn pair_from_str_panic() {
        Pair::from_str("BTC_WTF").unwrap();
    }

    #[test]
    fn pair_serde() {
        // Create a currency pair
        let pair = Pair(Symbol::BTC, Symbol::USD);

        // Serialize currency pair to JSON
        let serialized = serde_json::to_string(&pair).unwrap();
        assert_eq!(serialized, "[\"BTC\",\"USD\"]");

        // Deserialize currency pair
        let pair: Pair = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pair.0, Symbol::BTC);
        assert_eq!(pair.1, Symbol::USD);
    }

    #[test]
    fn pair_display() {
        let pair = Pair(Symbol::BTC, Symbol::USD);
        assert_eq!(pair.to_string(), "BTC_USD");
    }
}
