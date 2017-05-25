//! Cryptocurrency volume.

use std::fmt;

use symbol::Symbol;

/// Volume of a specified currency.
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Volume(pub Symbol, pub f64);

impl Volume {
    /// Currency symbol
    #[inline]
    pub fn currency(&self) -> Symbol {
        self.0
    }

    /// Amount of currency
    #[inline]
    pub fn amount(&self) -> f64 {
        self.1
    }
}

/// Displays currency volume
impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.1, self.0)
    }
}
