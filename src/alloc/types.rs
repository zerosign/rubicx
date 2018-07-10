use std::hash::Hash;

///
/// Allocator trait to allocate a value to
/// its corresponds SegmentFile.
///
/// it's not an "Allocator" for allocating memory - bear with me.
///
/// V: value type
/// O: output if correct
/// E: Error
///
/// Several logic load balance may applies in here, which consist of :
/// - scatter read & random based on size
/// - scatter read & random based on shards
///
pub trait Allocator<V, O, E>
where
    V: Sized + Hash,
{
    fn allocate(&mut self, value: V) -> Result<O, E>;
}
