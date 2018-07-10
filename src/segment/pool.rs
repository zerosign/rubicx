use crate::metric::data::{Metadata, Size};
use crate::{
    alloc::types::Allocator,
    cache::types::Cache,
    segment::{
        error::{ChecksumErr, SegmentErr},
        segment::SegmentFile,
        types::Append,
    },
};

use std::{collections::HashMap, hash::Hash, io, marker, path::Path, sync::atomic::AtomicU64};

///
/// SegmentPool will balance the write append requests
/// and as data physical layers.
///
/// A : Allocator
/// C : Cache
/// K : Key
/// V : Value
///
pub struct SegmentPool<A, C, K, V>
where
    K: Sized + Hash,
    A: Allocator<K, V, SegmentErr>,
    C: Cache<K, V>,
{
    data: HashMap<u64, SegmentFile>,
    metadata: Metadata,
    size: AtomicU64,
    allocator: Option<A>, // TODO: implement default allocator (scatter or linear)
    cache: Option<C>,     // TODO: implement default cache
    _key_type: marker::PhantomData<K>,
    _value_type: marker::PhantomData<V>,
}

impl<A, C, K, V> SegmentPool<A, C, K, V>
where
    K: Sized + Hash,
    A: Allocator<K, V, SegmentErr>,
    C: Cache<K, V>,
{
    pub fn new<P>(path: P, k: u64, size: Option<Size>) -> Result<Self, SegmentErr>
    where
        P: AsRef<Path> + Clone,
    {
        let mut segments = HashMap::<u64, SegmentFile>::new();

        for ii in 0..k {
            segments.insert(ii, SegmentFile::new(path.clone(), ii)?);
        }

        Ok(SegmentPool {
            data: segments,
            metadata: Metadata {
                base: path.as_ref().to_path_buf(),
                size: size.unwrap_or_default(),
            },
            size: AtomicU64::new(0),
            allocator: None,
            cache: None,
            _key_type: marker::PhantomData,
            _value_type: marker::PhantomData,
        })
    }
}

impl<A, C, K, V> Append for SegmentPool<A, C, K, V>
where
    K: Sized + Hash,
    A: Allocator<K, V, SegmentErr>,
    C: Cache<K, V>,
{
    fn append(&mut self, buffer: &[u8]) -> Result<usize, io::Error> {
        // unimplemented
        unimplemented!()
    }
}
