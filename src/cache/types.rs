use std::{hash::Hash, io};

use crate::ops::types::Fetch;
use crate::storage::types::Put;

///
/// base cache function
/// for load balance between memory backend storage and
/// file backend storage, or also can be used to
/// optimize index budgets.
///
pub trait Cache<K, V>: Fetch<K, V> + Put<K, V>
where
    K: Hash,
{
    type Time;

    fn fetch(self, key: K) -> io::Result<V>;

    fn expire(self, key: K, timeout: Option<Self::Time>) -> io::Result<Self::Time>;
}
