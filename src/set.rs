use core::cmp::Ordering;
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

impl<T> PartialEq for RangeSet<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RangeSet<T>) -> bool {
        self.rm == other.rm
    }
}

impl<T> Eq for RangeSet<T> where T: Eq {}

impl<T> PartialOrd for RangeSet<T>
where
    T: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &RangeSet<T>) -> Option<Ordering> {
        self.rm.partial_cmp(&other.rm)
    }
}

impl<T> Ord for RangeSet<T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &RangeSet<T>) -> Ordering {
        self.rm.cmp(&other.rm)
    }
}

impl<T> RangeSet<T>
where
    T: Ord + Clone,
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

    /// Clears the set, removing all elements.
    pub fn clear(&mut self) {
        self.rm.clear();
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.rm.len()
    }

    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.rm.is_empty()
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

    /// Gets an iterator over all the stored ranges that are
    /// either partially or completely overlapped by the given range.
    ///
    /// The iterator element type is `&Range<T>`.
    pub fn overlapping<'a>(&'a self, range: &'a Range<T>) -> Overlapping<T> {
        Overlapping {
            inner: self.rm.overlapping(range),
        }
    }

    /// Returns `true` if any range in the set completely or partially
    /// overlaps the given range.
    pub fn overlaps(&self, range: &Range<T>) -> bool {
        self.overlapping(range).next().is_some()
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

/// An iterator over all stored ranges partially or completely
/// overlapped by a given range.
///
/// This `struct` is created by the [`overlapping`] method on [`RangeSet`]. See its
/// documentation for more.
///
/// [`overlapping`]: RangeSet::overlapping
pub struct Overlapping<'a, T> {
    inner: crate::map::Overlapping<'a, T, ()>,
}

// `Overlapping` is always fused. (See definition of `next` below.)
impl<'a, T> core::iter::FusedIterator for Overlapping<'a, T> where T: Ord + Clone {}

impl<'a, T> Iterator for Overlapping<'a, T>
where
    T: Ord + Clone,
{
    type Item = &'a Range<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _v)| k)
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
    fn overlapping_partial_edges_complete_middle() {
        let mut range_map: RangeSet<u32> = RangeSet::new();

        // 0 1 2 3 4 5 6 7 8 9
        // ●---◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(0..2);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●---◌ ◌ ◌
        range_map.insert(5..7);

        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◇ ◌ ◌ ◌
        let query_range = 1..6;

        let mut overlapping = range_map.overlapping(&query_range);

        // Should yield partially overlapped range at start.
        assert_eq!(overlapping.next(), Some(&(0..2)));
        // Should yield completely overlapped range in middle.
        assert_eq!(overlapping.next(), Some(&(3..4)));
        // Should yield partially overlapped range at end.
        assert_eq!(overlapping.next(), Some(&(5..7)));
        // Gaps iterator should be fused.
        assert_eq!(overlapping.next(), None);
        assert_eq!(overlapping.next(), None);
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

    // const fn

    #[cfg(feature = "const_fn")]
    const _SET: RangeSet<u32> = RangeSet::new();
}
