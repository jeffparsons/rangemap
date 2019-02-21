/*!
[RangeMap](struct.RangeMap.html) is a map data structure whose keys are
stored as ranges. Contiguous and overlapping ranges
that map to the same value are coalesced into a single range.

A corresponding [RangeSet](struct.RangeSet.html) structure is also provided.
*/

mod map;
mod set;
mod std_ext;
#[cfg(test)]
mod stupid_range_map;

pub use map::RangeMap;
pub use set::RangeSet;
