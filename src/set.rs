use std::fmt::{self, Debug};
use std::ops::Range;

use crate::RangeMap;

#[derive(Clone)]
/// A set whose items are stored as (half-open) ranges bounded
/// inclusively below and exclusively above `(start..end)`.
///
/// See [`RangeMap`]'s documentation for more details.
///
/// [`RangeMap`]: struct.RangeMap.html
pub struct RangeSet<T> {
    rm: RangeMap<T, ()>,
}

impl<T> Default for RangeSet<T>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        RangeSet::new()
    }
}

impl<T> RangeSet<T>
where
    T: Ord + Clone,
{
    /// Makes a new empty `RangeSet`.
    pub fn new() -> Self {
        RangeSet {
            rm: RangeMap::new(),
        }
    }

    /// Returns a reference to the range covering the given key, if any.
    pub fn get(&self, value: &T) -> Option<&Range<T>> {
        self.rm.get_key_value(value).map(|(range, _)| range)
    }

    /// Returns `true` if any range in the set covers the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.rm.contains_key(value)
    }

    /// Gets an ordered iterator over all ranges,
    /// ordered by range.
    pub fn iter(&self) -> impl Iterator<Item = &Range<T>> {
        self.rm.iter().map(|(range, _v)| range)
    }

    /// Insert a range into the set.
    ///
    /// If the inserted range either overlaps or is immediately adjacent
    /// any existing range, then the ranges will be coalesced into
    /// a single contiguous range.
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn insert(&mut self, range: Range<T>) {
        self.rm.insert(range, ());
    }

    /// Removes a range from the set, if all or any of it was present.
    ///
    /// If the range to be removed _partially_ overlaps any ranges
    /// in the set, then those ranges will be contracted to no
    /// longer cover the removed range.
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn remove(&mut self, range: Range<T>) {
        self.rm.remove(range);
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the set.
    ///
    /// The iterator element type is `Range<T>`.
    ///
    /// NOTE: Calling `gaps` eagerly finds the first gap,
    /// even if the iterator is never consumed.
    pub fn gaps<'a>(&'a self, outer_range: &'a Range<T>) -> Gaps<'a, T> {
        Gaps {
            inner: self.rm.gaps(outer_range),
        }
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeSet does.
impl<T: Debug> Debug for RangeSet<T>
where
    T: Ord + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

pub struct Gaps<'a, T> {
    inner: crate::map::Gaps<'a, T, ()>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, T> std::iter::FusedIterator for Gaps<'a, T> where T: Ord + Clone {}

impl<'a, T> Iterator for Gaps<'a, T>
where
    T: Ord + Clone,
{
    type Item = Range<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait RangeSetExt<T> {
        fn to_vec(&self) -> Vec<Range<T>>;
    }

    impl<T> RangeSetExt<T> for RangeSet<T>
    where
        T: Ord + Clone,
    {
        fn to_vec(&self) -> Vec<Range<T>> {
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

    #[test]
    fn gaps_between_items_floating_inside_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_set.insert(5..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(3..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield gaps at start, between items,
        // and at end.
        assert_eq!(gaps.next(), Some(1..3));
        assert_eq!(gaps.next(), Some(4..5));
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }
    ///
    /// impl Debug
    ///

    #[test]
    fn set_debug_repr_looks_right() {
        let mut set: RangeSet<u32> = RangeSet::new();

        // Empty
        assert_eq!(format!("{:?}", set), "{}");

        // One entry
        set.insert(2..5);
        assert_eq!(format!("{:?}", set), "{2..5}");

        // Many entries
        set.insert(7..8);
        set.insert(10..11);
        assert_eq!(format!("{:?}", set), "{2..5, 7..8, 10..11}");
    }
}
