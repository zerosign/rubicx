use std::ops::{Index, IndexMut};

trait Bucket<T>: Index<T> + IndexMut<T>
where
    T: Sized + Copy + PartialEq,
{
    //
    // Lookup specific key in bucket.
    //
    fn lookup(&self, key: usize) -> Option<T>;

    //
    // In place mutation of specific key.
    //
    fn mutate(&mut self, key: usize) -> Result<&mut Option<&mut T>, BucketError>;

    //
    // Size of this bucket (total size of this bucket).
    //
    fn size(&self) -> usize;

    //
    // Actual size of this bucket (tenants).
    //
    fn usages(&self) -> usize;
}

//
// Base filter trait
//
trait Filter<T>
where
    T: Sized + Copy + Eq,
{
    //
    // Lookup value from filter.
    //
    fn lookup(&self, value: T) -> Option<T>;

    //
    // Insert value into filter.
    //
    fn insert(&mut self, value: T) -> Result<T, FilterError<T>>;

    //
    // Delete a value from filter.
    //
    fn delete(&mut self, value: T) -> Result<T, FilterError<T>>;
}

//
// Filter trait that define generic filter (point filter)
// behaviour.
//
// T must able to be hashed since current implementation
// relies on hash fn.
//
trait HashFilter<T>
where
    T: Hash + Sized + Copy + Eq,
{
    fn lookup(&self, value: T) -> Option<T>;

    fn insert(&mut self, value: T) -> Result<T, FilterError<T>>;

    fn delete(&mut self, value: T) -> Result<T, FilterError<T>>;
}
