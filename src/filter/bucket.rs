use std::sync::atomic::AtomicUsize;

//
// default segment size of this bucket.
// since Rust const fn are still in progress
// https://github.com/rust-lang/rust/issues/24111
//
const SEGMENT_SIZE: usize = 256;

//
// Data structure to hold segmented bucket.
//
// Since not every T has monoid or zero type
// of itself, we need to wrap it using Option,
// so we can differentiate between cell that
// has a value or not rather than some random
// value.
//
// We use vector of array since vector in rust
// didn't support insert into some point (randomly)
// you need populate the vector first, so we ends
// up using vector of array and use index propagation.
// This also solves if we need to make it expand
// dynamically.
//
pub struct FixedBucket<T>
where
    T: Default + Sized + Copy + Eq,
{
    inner: Vec<[Option<T>; SEGMENT_SIZE]>,
    tenant: AtomicUsize,
}

//
// resolves which segment that this key
// belongs to.
//
const fn sindex(key: usize, size: usize) -> usize {
    debug_assert!(size != 0);
    key / size
}

//
// resolves internal segment index where this
// key belongs to.
//
const fn index(key: usize, size: usize) -> usize {
    debug_assert!(size != 0);
    key % size
}

impl<T> Bucket<T> for FixedBucket<T>
where
    T: Default + Hash + Sized + Copy + PartialEq,
{
    //
    // create a bucket with capacity of `size`. Internally
    // we will find best fit for our segment size.
    //
    // - N : actual size of bucket
    // - d : segment size
    // - k : number of segment in bucket
    // - size : given target bucket size
    //
    // invariants;
    // - 0 < size <= N
    // - k * d = N
    //
    // We also need to populate the segment at least as much as k.
    // So, memory usage bounded to O(k * d) with operation of
    // O(k) of segment operations.
    //
    pub fn with_capacity(size: usize) -> FixedBucket<T> {
        debug_assert!(size != 0);

        let size = (size / SEGMENT_SIZE) + 1;

        let mut data: Vec<[Option<T>; SEGMENT_SIZE]> = Vec::with_capacity(size);

        // size of data should never be a 0
        debug_assert!(data.len() != 0);

        for ii in 0..size {
            data.push([None; SEGMENT_SIZE]);
        }

        FixedBucket { inner: data }
    }

    //
    // Get value from bucket based on key.
    //
    // returns non mutable value (copied).
    //
    // None could means both :
    // - segment which holds the key doesn't exists
    // - value doesn't exists in their "should be" segment
    //
    pub fn lookup(&self, key: usize) -> Option<T> {
        let sidx = sindex(key, self.len());
        let idx = index(key, self.len());

        match self.inner.get(sidx) {
            Some(segment) => match segment.get(idx) {
                Some(Some(data)) => Some(*data),
                _ => None,
            },
            _ => None,
        }
    }

    //
    // Get mutable value ref from bucket based on key.
    //
    // None could means both :
    // - segment which holds the key doesn't exists
    // - value doesn't exists in their "should be" segment
    //
    pub fn mutate(&mut self, key: usize) -> Option<&mut T> {
        let sidx = sindex(key, self.len());
        let idx = index(key, self.len());

        match self.inner.get_mut(sidx) {
            Some(mut segment) => match segment.get_mut(idx) {
                // need to unbox and borrow the ref mut
                Some(Some(ref mut data)) => Some(data),
                _ => None,
            },
            _ => None,
        }
    }

    //
    // Get total length of this bucket.
    //
    pub fn size(&self) -> usize {
        self.inner.len() * SEGMENT_SIZE
    }
}
