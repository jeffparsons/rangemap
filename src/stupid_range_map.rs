use std::collections::BTreeMap;
use std::ops::Range;

use super::RangeMap;

// A simple but infeasibly slow and memory-hungry
// version of `RangeMap` for testing.
//
// Only understands `u32` keys, so that we don't
// have to be generic over `step`. This is just for
// testing, so it's fine.
#[derive(Eq, PartialEq, Debug)]
pub struct StupidU32RangeMap<V> {
    // Inner B-Tree map. Stores values and their keys
    // directly rather than as ranges.
    btm: BTreeMap<u32, V>,
}

impl<V> StupidU32RangeMap<V>
where
    V: Eq + Clone,
{
    pub fn new() -> StupidU32RangeMap<V> {
        StupidU32RangeMap {
            btm: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, range: Range<u32>, value: V) {
        for k in range {
            self.btm.insert(k, value.clone());
        }
    }
}

impl<V> From<RangeMap<u32, V>> for StupidU32RangeMap<V>
where
    V: Eq + Clone,
{
    fn from(range_map: RangeMap<u32, V>) -> Self {
        let mut stupid = Self::new();
        for (range, value) in range_map.iter() {
            stupid.insert(range.clone(), value.clone());
        }
        stupid
    }
}
