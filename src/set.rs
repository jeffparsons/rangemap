use core::fmt::{self, Debug};
use core::iter::FromIterator;
use core::ops::Range;
use core::prelude::v1::*;

#[cfg(feature = "serde1")]
use core::marker::PhantomData;
#[cfg(feature = "serde1")]
use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

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
    /// If the start and end of the outer range are the same
    /// and it does not overlap any stored range, then a single
    /// empty gap will be returned.
    ///
    /// The iterator element type is `Range<T>`.
    pub fn gaps<'a>(&'a self, outer_range: &'a Range<T>) -> Gaps<'a, T> {
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
    type Item = &'a Range<T>;

    fn next(&mut self) -> Option<&'a Range<T>> {
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
    type Item = Range<T>;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.rm.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = Range<T>;
    fn next(&mut self) -> Option<Range<T>> {
        self.inner.next().map(|(range, _)| range)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
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

impl<T> FromIterator<Range<T>> for RangeSet<T>
where
    T: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
        let mut range_set = RangeSet::new();
        range_set.extend(iter);
        range_set
    }
}

impl<T> Extend<Range<T>> for RangeSet<T>
where
    T: Ord + Clone,
{
    fn extend<I: IntoIterator<Item = Range<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |range| {
            self.insert(range);
        })
    }
}

#[cfg(feature = "serde1")]
impl<T> Serialize for RangeSet<T>
where
    T: Ord + Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.rm.btm.len()))?;
        for range in self.iter() {
            seq.serialize_element(&(&range.start, &range.end))?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Deserialize<'de> for RangeSet<T>
where
    T: Ord + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RangeSetVisitor::new())
    }
}

#[cfg(feature = "serde1")]
struct RangeSetVisitor<T> {
    marker: PhantomData<fn() -> RangeSet<T>>,
}

#[cfg(feature = "serde1")]
impl<T> RangeSetVisitor<T> {
    fn new() -> Self {
        RangeSetVisitor {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Visitor<'de> for RangeSetVisitor<T>
where
    T: Ord + Clone + Deserialize<'de>,
{
    type Value = RangeSet<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RangeSet")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut range_set = RangeSet::new();
        while let Some((start, end)) = access.next_element()? {
            range_set.insert(start..end);
        }
        Ok(range_set)
    }
}

/// An iterator over all ranges not covered by a `RangeSet`.
///
/// This `struct` is created by the [`gaps`] method on [`RangeSet`]. See its
/// documentation for more.
///
/// [`gaps`]: RangeSet::gaps
pub struct Gaps<'a, T> {
    inner: crate::map::Gaps<'a, T, ()>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, T> core::iter::FusedIterator for Gaps<'a, T> where T: Ord + Clone {}

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
    use alloc::{format, vec, vec::Vec};

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

    #[test]
    fn new_same_value_immediately_following_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_set.insert(3..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_different_value_immediately_following_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3, );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_set.insert(3..5 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_same_value_overlapping_end_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..4 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_set.insert(3..5 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_different_value_overlapping_end_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..4 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_set.insert(3..5 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_same_value_immediately_preceding_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_set.insert(3..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_different_value_immediately_preceding_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_set.insert(3..5 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }


    #[test]
    fn new_same_value_wholly_inside_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_set.insert(1..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![(1..5)]);
    }

    #[test]
    fn new_different_value_wholly_inside_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------◇ ◌ ◌ ◌ ◌
        range_set.insert(1..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
        assert_eq!(
            range_set.to_vec(),
            vec![(1..5)]
        );
    }    

    #[test]
    fn replace_at_end_of_existing_range_should_coalesce() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_set.insert(3..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_set.insert(3..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_set.to_vec(), vec![1..5 ]);
    }

    // Omitted maps dense test.

    //
    // Get* tests
    //

    #[test]
    fn get() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(0..50 );
        assert_eq!(range_set.get(&49), Some(&(0..50)));
        assert_eq!(range_set.get(&50), None);
    }

    //
    // Removal tests
    //

    #[test]
    fn remove_from_empty_map() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.remove(0..50);
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn remove_non_covered_range_before_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(0..25);
        assert_eq!(range_set.to_vec(), vec![(25..75)]);
    }

    #[test]
    fn remove_non_covered_range_after_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(75..100);
        assert_eq!(range_set.to_vec(), vec![(25..75)]);
    }

    #[test]
    fn remove_overlapping_start_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(0..30);
        assert_eq!(range_set.to_vec(), vec![(30..75)]);
    }

    #[test]
    fn remove_middle_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(30..70);
        assert_eq!(range_set.to_vec(), vec![(25..30), (70..75)]);
    }

        #[test]
    fn remove_overlapping_end_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(70..100);
        assert_eq!(range_set.to_vec(), vec![(25..70)]);
    }

    #[test]
    fn remove_exactly_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(25..75);
        assert_eq!(range_set.to_vec(), vec![]);
    }
    
    #[test]
    fn remove_superset_of_stored() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(25..75);
        range_set.remove(0..100);
        assert_eq!(range_set.to_vec(), vec![]);
    }

    //
    // Gaps tests
    //
    
    #[test]
    fn whole_range_is_a_gap() {
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        let range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn whole_range_is_covered_exactly() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_set.insert(1..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◇ ◌ ◌ ◌
        let outer_range = 1..6;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_before_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_touching_start_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_set.insert(1..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_overlapping_start_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_set.insert(1..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield from the end of the stored item
        // to the end of the outer range.
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_starting_at_start_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_set.insert(5..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield from the item onwards.
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn items_floating_inside_outer_range() {
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

    #[test]
    fn item_ending_at_end_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ●-◌ ◌
        range_set.insert(7..8);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield from the start of the outer range
        // up to the start of the stored item.
        assert_eq!(gaps.next(), Some(5..7));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_overlapping_end_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●---◌ ◌ ◌ ◌
        range_set.insert(4..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌
        let outer_range = 2..5;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield from the start of the outer range
        // up to the start of the stored item.
        assert_eq!(gaps.next(), Some(2..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_touching_end_of_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●-------◌ ◌
        range_set.insert(4..8);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_after_outer_range() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ●---◌ ◌
        range_set.insert(6..7);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_away_from_both_sides() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
        range_set.insert(5..7);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield a gap covering the zero-width outer range.
        assert_eq!(gaps.next(), Some(4..4));
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_touching_both_sides() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌
        range_set.insert(4..6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_item_straddling() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..5);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn no_empty_gaps() {
        // Make two ranges different values so they don't
        // get coalesced.
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
        range_set.insert(4..5 );
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(3..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield gaps at start and end, but not between the
        // two touching items. (4 is covered, so there should be no gap.)
        assert_eq!(gaps.next(), Some(1..3));
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn adjacent_small_items() {
        // Items two items next to each other at the start, and at the end.
        let mut range_set: RangeSet<u32> = RangeSet::new();
        range_set.insert(0..1);
        range_set.insert(1..2);
        range_set.insert(253..254);
        range_set.insert(254..255);

        let outer_range = 0..255;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield one big gap in the middle.
        assert_eq!(gaps.next(), Some(2..253));
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

    // impl Serialize

    #[cfg(feature = "serde1")]
    #[test]
    fn serialization() {
        let mut range_set: RangeSet<u32> = RangeSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
        range_set.insert(5..7);
        let output = serde_json::to_string(&range_set).expect("Failed to serialize");
        assert_eq!(output, "[[1,3],[5,7]]");
    }

    // impl Deserialize

    #[cfg(feature = "serde1")]
    #[test]
    fn deserialization() {
        let input = "[[1,3],[5,7]]";
        let range_set: RangeSet<u32> = serde_json::from_str(input).expect("Failed to deserialize");
        let reserialized = serde_json::to_string(&range_set).expect("Failed to re-serialize");
        assert_eq!(reserialized, input);
    }
}
