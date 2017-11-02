
use std::str::FromStr;
use std::convert::From;
use serde_json::Value;
use serde_json::value::Index;
use util::try_opt;
use core::errors::*;

#[inline]
pub fn get_array<'a, I: Index>(event: &'a Value, index: I) -> Result<&'a Vec<Value>> {
    let event = try_opt(event.get(index))?;
    let event = try_opt(event.as_array())?;
    Ok(event)
}

#[inline]
pub fn get_object<'a>(
    objects: &'a Vec<Value>,
    index: usize,
) -> Result<&'a ::serde_json::Map<String, Value>> {
    let object = try_opt(objects.get(index))?;
    let object = try_opt(object.as_object())?;
    Ok(object)
}

#[inline]
pub fn get_i64<I: Index>(event: &Value, index: I) -> Result<i64> {
    let event = try_opt(event.get(index))?;
    let event = try_opt(event.as_i64())?;
    Ok(event)
}

#[inline]
pub fn get_str<'a, I: Index>(event: &'a Value, index: I) -> Result<&'a str> {
    let event = try_opt(event.get(index))?;
    let event = try_opt(event.as_str())?;
    Ok(event)
}

#[inline]
pub fn parse_nth_str<T: FromStr>(event: &Value, index: usize) -> Result<T>
where
    Error: From<<T as FromStr>::Err>,
{
    match event.get(index) {
        Some(value) => parse_str::<T>(value),
        None => Err(ErrorKind::ValueNotFound.into()),
    }
}

#[inline]
pub fn parse_str<T: FromStr>(value: &Value) -> Result<T>
where
    Error: From<<T as FromStr>::Err>,
{
    match value.as_str() {
        Some(value) => value.parse::<T>().map_err(|e| e.into()),
        None => Err(ErrorKind::ValueNotFound.into()),
    }
}
