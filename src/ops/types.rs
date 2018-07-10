use std::hash::Hash;
use std::sync::atomic::AtomicU64;

pub trait Filter<V, E>
where
    V: Sized + Hash,
{
    fn filter(&mut self, value: V) -> Result<bool, E>;
}

pub trait Fetch<K, V>: Sized
where
    K: Sized + Hash,
    V: Sized,
{
    type Error;

    fn fetch(&mut self, key: K) -> Result<V, Self::Error>;
}

pub trait Delete<K>: Sized
where
    K: Sized + Hash,
{
    type Error;

    fn delete(&mut self, key: K) -> Result<(), Self::Error>;
}
