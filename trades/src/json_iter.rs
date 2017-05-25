
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use lmdb::{CursorIter, Database};
use lmdb::core::{CursorIterator, MdbResult};

/// Database JSON iterator.
pub struct JsonIter<'a, T: 'a> {
    inner: CursorIterator<'a, CursorIter>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> JsonIter<'a, T>
    where T: DeserializeOwned + 'a
{
    pub fn new(db: &'a Database<'a>) -> MdbResult<JsonIter<'a, T>> {
        Ok(JsonIter {
            inner: db.iter()?,
            phantom: PhantomData,
        })
    }
}

impl<'a, T> Iterator for JsonIter<'a, T>
    where T: DeserializeOwned + 'a
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(item) => {
                let body = item.get_value::<&str>();
                let value = ::serde_json::from_str::<Self::Item>(body).unwrap();
                Some(value)
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
