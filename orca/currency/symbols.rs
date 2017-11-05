// Currency symbols utilities.

use std::convert::TryFrom;
use protobuf::ProtobufEnum;

use ::currency::Symbol;

/// Tries to convert string to a currency `Symbol`.
impl<'a> TryFrom<&'a str> for Symbol {
    type Error = ::core::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        for c in <Symbol as ProtobufEnum>::values() {
            if <Symbol as ProtobufEnum>::descriptor(c).name() == name {
                return Ok(c.clone());
            }
        }
        Err(
            ::core::ErrorKind::UnknownCurrency(name.to_owned()).into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn symbol_debug() {
        let c = Symbol::try_from("BTC").unwrap();
        assert_eq!("BTC".to_owned(), format!("{:?}", c));
    }

    #[bench]
    fn symbol_try_from_str(b: &mut Bencher) {
        b.iter(|| { Symbol::try_from("BTC").unwrap(); });
    }

    #[bench]
    fn format_debug(b: &mut Bencher) {
        let c = Symbol::try_from("BTC").unwrap();
        b.iter(|| format!("{:?}", c));
    }
}
