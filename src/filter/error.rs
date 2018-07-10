use std::fmt;

//
// Error enum for Bucket operations.
//
#[derive(Debug)]
pub enum BucketError<T>
where
    T: Sized + fmt::Debug,
{
    SegmentNotFound(usize, T),
    IndexNotFound(usize, usize, T),
}

//
// Error enum to give specific error for Filter operations.
//
#[derive(Debug)]
pub enum FilterError<T>
where
    T: Sized + fmt::Debug,
{
    FilterFull(usize),
    UnknownError(T),
}
