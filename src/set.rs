use std::ops::Range;

use crate::RangeMap;

#[derive(Clone)]
pub struct RangeSet<K> {
    rm: RangeMap<K, ()>,
}

impl<K> Default for RangeSet<K>
where
    K: Ord + Clone,
{
    fn default() -> Self {
        RangeSet::new()
    }
}

impl<K> RangeSet<K>
where
    K: Ord + Clone,
{
    pub fn new() -> Self {
        RangeSet {
            rm: RangeMap::new(),
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.rm.contains_key(key)
    }

    /// Gets an ordered iterator over all key ranges,
    /// ordered by key range.
    pub fn iter(&self) -> impl Iterator<Item = &Range<K>> {
        self.rm.iter().map(|(range, _v)| range)
    }

    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn insert(&mut self, range: Range<K>) {
        self.rm.insert(range, ());
    }

    /// Removes a range from the set, if all or any of it was present.
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn remove(&mut self, range: Range<K>) {
        self.rm.remove(range);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait RangeMapExt<K> {
        fn to_vec(&self) -> Vec<Range<K>>;
    }

    impl<K> RangeMapExt<K> for RangeSet<K>
    where
        K: Ord + Clone,
    {
        fn to_vec(&self) -> Vec<Range<K>> {
            self.iter().cloned().collect()
        }
    }

    #[test]
    fn empty_set_is_empty() {
        let range_set: RangeSet<u32> = RangeSet::new();
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(0..50);
        assert_eq!(range_set.to_vec(), vec![0..50]);
    }

    #[test]
    fn remove_partially_overlapping() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(0..50);
        range_set.remove(25..75);
        assert_eq!(range_set.to_vec(), vec![0..25]);
    }
}
