//! Orca core utilities.

pub mod parse;

mod pair;
pub use self::pair::*;

mod future;
pub use self::future::boxfuture;
pub use self::future::FutureExt;


use core::errors::{Result, ErrorKind};

#[inline]
pub fn try_opt<T: Sized>(opt: Option<T>) -> Result<T> {
    match opt {
        Some(value) => Ok(value),
        None => Err(ErrorKind::ValueNotFound.into()),
    }
}
