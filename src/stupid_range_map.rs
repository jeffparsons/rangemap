use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use super::{RangeInclusiveMap, RangeMap};

// A simple but infeasibly slow and memory-hungry
// version of `RangeInclusiveMap` for testing.
//
// Only understands `u32` keys, so that we don't
// have to be generic over `step`. This is just for
// testing, so it's fine.
//
// Note that even though this mirrors the semantics
// of `RangeInclusiveMap`, it can also be used to
// test `RangeMap` just fine.
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

    pub fn insert(&mut self, range: RangeInclusive<u32>, value: V) {
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
            // Convert to inclusive end.
            // (This is only valid because u32 has
            // the "successor function".)
            //
            // NOTE: Clippy's `range_minus_one` lint is a bit overzealous here,
            // because we _can't_ pass an open-ended range to `insert`.
            #[allow(clippy::range_minus_one)]
            stupid.insert(range.start..=(range.end - 1), value.clone());
        }
        stupid
    }
}

impl<V> From<RangeInclusiveMap<u32, V>> for StupidU32RangeMap<V>
where
    V: Eq + Clone,
{
    fn from(range_inclusive_map: RangeInclusiveMap<u32, V, u32>) -> Self {
        let mut stupid = Self::new();
        for (range, value) in range_inclusive_map.iter() {
            stupid.insert(range.clone(), value.clone());
        }
        stupid
    }
}
