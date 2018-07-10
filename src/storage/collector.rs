///
/// Collector will scan graveyard and remerge remaining
/// blocks in SegmentFile into new SegmentFile based on Allocator.
///
/// TODO: we might want to use rayon crate to deal with this effeciently.
///
pub trait Collector;
