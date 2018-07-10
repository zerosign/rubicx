use crate::segment::{pool::SegmentPool, types::Append};
use std::{hash::Hash, io};

///
///
///
pub trait Delete<K, V>: Sized
where
    K: Sized + Hash,
    V: Sized,
{
    type Error;

    fn delete(&mut self, idx: K) -> Result<(), Self::Error>;
}

///
/// Put operation as one of Random access operations.
///
pub trait Put<K, V>: Sized
where
    K: Sized + Hash,
    V: Sized,
{
    type Error;

    fn put(&mut self, idx: K, value: V) -> Result<(), Self::Error>;
}

///
/// Graveyard will holds its own segment files.
///
/// In logical layers, graveyard would deals with
/// all deleted values.
///
/// It has the same abstraction over Storage, but
/// it didn't support random access. This type of storage
/// only do append only or scan.
///
/// Collector will scan the graveyard to do the cleanup in background threads.
///
pub struct Graveyard<A, C, K, V>
where
    K: Sized + Hash,
    V: Sized,
    A: Allocator<K, V, SegmentErr>,
    C: Cache<K, V>,
{
    data: SegmentPool<A, C, K, V>,
}

impl Append for Graveyard {
    fn append(&mut self, buffer: &[u8]) -> Result<usize, SegmentErr> {}
}

pub struct Storage<T>
where
    T: Sized + Hash,
{
    inner: SegmentPool,
    bin: Graveyard,
}

impl<T> Put for Storage {}
