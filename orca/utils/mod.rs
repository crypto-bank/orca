
mod pair;
pub mod parse;
pub use self::pair::*;

use core::errors::*;

#[inline]
pub fn try_opt<T: Sized>(opt: Option<T>) -> Result<T> {
    match opt {
        Some(value) => Ok(value),
        None => Err(ErrorKind::ValueNotFound.into()),
    }
}
