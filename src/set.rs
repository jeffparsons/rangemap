use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::iter::FromIterator;
use core::prelude::v1::*;

use crate::{RangeMap, RangeTrait};

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

impl<T, I> Default for RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    fn default() -> Self {
        RangeSet::new()
    }
}

impl<T, I> PartialEq for RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    fn eq(&self, other: &RangeSet<T>) -> bool {
        self.rm == other.rm
    }
}

impl<T, I> Eq for RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
}

impl<T, I> PartialOrd for RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    #[inline]
    fn partial_cmp(&self, other: &RangeSet<T>) -> Option<Ordering> {
        self.rm.partial_cmp(&other.rm)
    }
}

impl<T, I> Ord for RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    #[inline]
    fn cmp(&self, other: &RangeSet<T>) -> Ordering {
        self.rm.cmp(&other.rm)
    }
}

impl<T, I> RangeSet<T>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    /// Makes a new empty `RangeSet`.
    #[cfg(feature = "const_fn")]
    pub const fn new() -> Self {
        RangeSet {
            rm: RangeMap::new(),
        }
    }

    /// Makes a new empty `RangeSet`.
    #[cfg(not(feature = "const_fn"))]
    pub fn new() -> Self {
        RangeSet {
            rm: RangeMap::new(),
        }
    }

    /// Returns a reference to the range covering the given key, if any.
    pub fn get(&self, value: &I) -> Option<&T> {
        self.rm.get_key_value(value).map(|(range, _)| range)
    }

    /// Returns `true` if any range in the set covers the specified value.
    pub fn contains(&self, value: &I) -> bool {
        self.rm.contains_key(value)
    }

    /// Gets an ordered iterator over all ranges,
    /// ordered by range.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.rm.iter(),
        }
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
    pub fn insert(&mut self, range: T) {
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
    pub fn remove(&mut self, range: T) {
        self.rm.remove(range);
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the set.
    ///
    /// If the start and end of the outer range are the same
    /// and it does not overlap any stored range, then a single
    /// empty gap will be returned.
    ///
    /// The iterator element type is `Range<T>`.
    pub fn gaps<'a>(&'a self, outer_range: &'a T) -> Gaps<'a, T, I> {
        Gaps {
            inner: self.rm.gaps(outer_range),
        }
    }
}

/// An iterator over the ranges of a `RangeSet`.
///
/// This `struct` is created by the [`iter`] method on [`RangeSet`]. See its
/// documentation for more.
///
/// [`iter`]: RangeSet::iter
pub struct Iter<'a, T> {
    inner: super::map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next().map(|(range, _)| range)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// An owning iterator over the ranges of a `RangeSet`.
///
/// This `struct` is created by the [`into_iter`] method on [`RangeSet`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
pub struct IntoIter<T> {
    inner: super::map::IntoIter<T, ()>,
}

impl<T> IntoIterator for RangeSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.rm.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.inner.next().map(|(range, _)| range)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeSet does.
impl<T, I> Debug for RangeSet<T>
where
    T: RangeTrait<A = I> + Clone + Debug,
    I: Clone + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T, I> FromIterator<T> for RangeSet<T>
where
    T: RangeTrait<A = I> + Clone,
    I: Clone + Ord,
{
    fn from_iter<M: IntoIterator<Item = T>>(iter: M) -> Self {
        let mut range_set = RangeSet::new();
        range_set.extend(iter);
        range_set
    }
}

impl<T, I> Extend<T> for RangeSet<T>
where
    T: RangeTrait<A = I> + Clone,
    I: Clone + Ord,
{
    fn extend<M: IntoIterator<Item = T>>(&mut self, iter: M) {
        iter.into_iter().for_each(move |range| {
            self.insert(range);
        })
    }
}

/// An iterator over all ranges not covered by a `RangeSet`.
///
/// This `struct` is created by the [`gaps`] method on [`RangeSet`]. See its
/// documentation for more.
///
/// [`gaps`]: RangeSet::gaps
pub struct Gaps<'a, T, I> {
    inner: crate::map::Gaps<'a, T, I, ()>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, T, I> core::iter::FusedIterator for Gaps<'a, T, I>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
}

impl<'a, T, I> Iterator for Gaps<'a, T, I>
where
    T: Clone + RangeTrait<A = I>,
    I: Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec, vec::Vec};
    use core::ops::Range;

    trait RangeSetExt<T> {
        fn to_vec(&self) -> Vec<T>;
    }

    impl<T, I> RangeSetExt<T> for RangeSet<T>
    where
        T: RangeTrait<A = I> + Clone,
        I: Clone + Ord,
    {
        fn to_vec(&self) -> Vec<T> {
            self.iter().cloned().collect()
        }
    }

    #[test]
    fn empty_set_is_empty() {
        let range_set: RangeSet<Range<u32>> = RangeSet::new();
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_set: RangeSet<Range<u32>> = RangeSet::new();
        range_set.insert(0..50);
        assert_eq!(range_set.to_vec(), vec![0..50]);
    }

    #[test]
    fn remove_partially_overlapping() {
        let mut range_set: RangeSet<Range<u32>> = RangeSet::new();
        range_set.insert(0..50);
        range_set.remove(25..75);
        assert_eq!(range_set.to_vec(), vec![0..25]);
    }

    #[test]
    fn gaps_between_items_floating_inside_outer_range() {
        let mut range_set: RangeSet<Range<u32>> = RangeSet::new();
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
        let mut set: RangeSet<Range<u32>> = RangeSet::new();

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
