use super::range_wrapper::RangeStartWrapper;
use crate::std_ext::*;
use alloc::collections::BTreeMap;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::ops::{Index, Range};
use core::prelude::v1::*;

/// A map whose keys are stored as (half-open) ranges bounded
/// inclusively below and exclusively above `(start..end)`.
///
/// Contiguous and overlapping ranges that map to the same value
/// are coalesced into a single range.
#[derive(Clone)]
pub struct RangeMap<K, V> {
    // Wrap ranges so that they are `Ord`.
    // See `range_wrapper.rs` for explanation.
    btm: BTreeMap<RangeStartWrapper<K>, V>,
}

// TODO: note differences with std (borrow Q stuff)

impl<K, V> RangeMap<K, V>
where
    K: Ord + Clone,
    V: Eq + Clone,
{
    /// Makes a new, empty `RangeMap`.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    ///
    /// // entries can now be inserted into the empty map
    /// map.insert(0..1, "a");
    /// ```
    pub fn new() -> Self {
        RangeMap {
            btm: BTreeMap::new(),
        }
    }

    /// Clears the map, removing all elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut a = RangeMap::new();
    /// a.insert(0..1, "a");
    /// a.clear();
    /// assert!(a.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.btm.clear()
    }

    /// Returns a reference to the value corresponding to the given key,
    /// if the key is covered by any range in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(0..1, "a");
    /// assert_eq!(map.get(&0), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_key_value(key).map(|(_range, value)| value)
    }

    /// Returns the range-value pair (as a pair of references) corresponding
    /// to the given key, if the key is covered by any range in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(0..1, "a");
    /// assert_eq!(map.get_key_value(&0), Some((&(0..1), &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    pub fn get_key_value(&self, k: &K) -> Option<(&Range<K>, &V)> {
        use core::ops::Bound;

        // The only stored range that could contain the given key is the
        // last stored range whose start is less than or equal to this key.
        let key_as_start = RangeStartWrapper::new(k.clone()..k.clone());
        self.btm
            .range((Bound::Unbounded, Bound::Included(key_as_start)))
            .next_back()
            .filter(|(range_start_wrapper, _value)| {
                // Does the only candidate range contain
                // the requested key?
                range_start_wrapper.range.contains(k)
            })
            .map(|(range_start_wrapper, value)| (&range_start_wrapper.range, value))
    }

    /// Returns `true` if any range in the map covers the specified key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(0..1, "a");
    /// assert_eq!(map.contains_key(&0), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    // TODO: Implement: Should this mutate the entire containing range (the
    // implementation as a map might see it), or should it add a new point
    // range?). Because of this ambiguity, maybe this function *shouldn't* be
    // implemented, in favor of get_range() or insert() or update_range(), etc?

    // /// Returns a mutable reference to the value corresponding to the key.
    // ///
    // /// The key may be any borrowed form of the map's key type, but the ordering
    // /// on the borrowed form *must* match the ordering on the key type.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // ///
    // /// ```
    // /// use std::collections::BTreeMap;
    // ///
    // /// let mut map = BTreeMap::new();
    // /// map.insert(1, "a");
    // /// if let Some(x) = map.get_mut(&1) {
    // ///     *x = "b";
    // /// }
    // /// assert_eq!(map[&1], "b");
    // /// ```
    // BTreeMap Implementation
    // pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    // where
    //     K: Borrow<Q> + Ord,
    //     Q: Ord,
    // {
    //     let root_node = self.root.as_mut()?.borrow_mut();
    //     match root_node.search_tree(key) {
    //         Found(handle) => Some(handle.into_val_mut()),
    //         GoDown(_) => None,
    //     }
    // }

    // TODO: for BTreeMap parity, return value on overlap? What to do with multiple overlaps?
    // Maybe return another RangeMap with the removed segments? But that would allocate,
    // so that could be opt-in via a similar function (insert_returning, or something)?

    /// Insert a pair of key range and value into the map.
    ///
    /// If the inserted range partially or completely overlaps any
    /// existing range in the map, then the existing range (or ranges) will be
    /// partially or completely replaced by the inserted range.
    ///
    /// If the inserted range either overlaps or is immediately adjacent
    /// any existing range _mapping to the same value_, then the ranges
    /// will be coalesced into a single contiguous range.
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(0..4, "a");
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(2..6, "b");
    /// assert_eq!(map[&1], "a");
    /// assert_eq!(map[&3], "b");
    /// ```
    pub fn insert(&mut self, range: Range<K>, value: V) {
        use core::ops::Bound;

        // We don't want to have to make empty ranges make sense;
        // they don't represent anything meaningful in this structure.
        assert!(range.start < range.end);

        // Wrap up the given range so that we can "borrow"
        // it as a wrapper reference to either its start or end.
        // See `range_wrapper.rs` for explanation of these hacks.
        let mut new_range_start_wrapper: RangeStartWrapper<K> = RangeStartWrapper::new(range);
        let new_value = value;

        // Is there a stored range either overlapping the start of
        // the range to insert or immediately preceding it?
        //
        // If there is any such stored range, it will be the last
        // whose start is less than or equal to the start of the range to insert,
        // or the one before that if both of the above cases exist.
        let mut candidates = self
            .btm
            .range((Bound::Unbounded, Bound::Included(&new_range_start_wrapper)))
            .rev()
            .take(2)
            .filter(|(stored_range_start_wrapper, _stored_value)| {
                // Does the candidate range either overlap
                // or immediately precede the range to insert?
                // (Remember that it might actually cover the _whole_
                // range to insert and then some.)
                stored_range_start_wrapper
                    .range
                    .touches(&new_range_start_wrapper.range)
            });
        if let Some(mut candidate) = candidates.next() {
            // Or the one before it if both cases described above exist.
            if let Some(another_candidate) = candidates.next() {
                candidate = another_candidate;
            }
            let (stored_range_start_wrapper, stored_value) =
                (candidate.0.clone(), candidate.1.clone());
            self.adjust_touching_ranges_for_insert(
                stored_range_start_wrapper,
                stored_value,
                &mut new_range_start_wrapper.range,
                &new_value,
            );
        }

        // Are there any stored ranges whose heads overlap or immediately
        // follow the range to insert?
        //
        // If there are any such stored ranges (that weren't already caught above),
        // their starts will fall somewhere after the start of the range to insert,
        // and on or before its end.
        //
        // This time around, if the latter holds, it also implies
        // the former so we don't need to check here if they touch.
        //
        // REVISIT: Possible micro-optimisation: `impl Borrow<T> for RangeStartWrapper<T>`
        // and use that to search here, to avoid constructing another `RangeStartWrapper`.
        let new_range_end_as_start = RangeStartWrapper::new(
            new_range_start_wrapper.range.end.clone()..new_range_start_wrapper.range.end.clone(),
        );
        while let Some((stored_range_start_wrapper, stored_value)) = self
            .btm
            .range((
                Bound::Included(&new_range_start_wrapper),
                Bound::Included(&new_range_end_as_start),
            ))
            .next()
        {
            // One extra exception: if we have different values,
            // and the stored range starts at the end of the range to insert,
            // then we don't want to keep looping forever trying to find more!
            #[allow(clippy::suspicious_operation_groupings)]
            if stored_range_start_wrapper.range.start == new_range_start_wrapper.range.end
                && *stored_value != new_value
            {
                // We're beyond the last stored range that could be relevant.
                // Avoid wasting time on irrelevant ranges, or even worse, looping forever.
                // (`adjust_touching_ranges_for_insert` below assumes that the given range
                // is relevant, and behaves very poorly if it is handed a range that it
                // shouldn't be touching.)
                break;
            }

            let stored_range_start_wrapper = stored_range_start_wrapper.clone();
            let stored_value = stored_value.clone();

            self.adjust_touching_ranges_for_insert(
                stored_range_start_wrapper,
                stored_value,
                &mut new_range_start_wrapper.range,
                &new_value,
            );
        }

        // Insert the (possibly expanded) new range, and we're done!
        self.btm.insert(new_range_start_wrapper, new_value);
    }

    // TODO: similar to insert(), can we return values? overlaps?
    /// Removes a range from the map, if all or any of it was present.
    ///
    /// If the range to be removed _partially_ overlaps any ranges
    /// in the map, then those ranges will be contracted to no
    /// longer cover the removed range.
    ///
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rangemap::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(0..10, "a");
    /// map.remove(3..5);
    /// assert_eq!(map.get(&0), Some("a"));
    /// assert_eq!(map.get(&3), None);
    /// assert_eq!(map.get(&5), Some("a"));
    /// assert_eq!(map.get(&8), Some("a"));
    /// ```
    pub fn remove(&mut self, range: Range<K>) {
        use core::ops::Bound;

        // We don't want to have to make empty ranges make sense;
        // they don't represent anything meaningful in this structure.
        assert!(range.start < range.end);

        let range_start_wrapper: RangeStartWrapper<K> = RangeStartWrapper::new(range);
        let range = &range_start_wrapper.range;

        // Is there a stored range overlapping the start of
        // the range to insert?
        //
        // If there is any such stored range, it will be the last
        // whose start is less than or equal to the start of the range to insert.
        if let Some((stored_range_start_wrapper, stored_value)) = self
            .btm
            .range((Bound::Unbounded, Bound::Included(&range_start_wrapper)))
            .next_back()
            .filter(|(stored_range_start_wrapper, _stored_value)| {
                // Does the only candidate range overlap
                // the range to insert?
                stored_range_start_wrapper.range.overlaps(&range)
            })
            .map(|(stored_range_start_wrapper, stored_value)| {
                (stored_range_start_wrapper.clone(), stored_value.clone())
            })
        {
            self.adjust_overlapping_ranges_for_remove(
                stored_range_start_wrapper,
                stored_value,
                &range,
            );
        }

        // Are there any stored ranges whose heads overlap the range to insert?
        //
        // If there are any such stored ranges (that weren't already caught above),
        // their starts will fall somewhere after the start of the range to insert,
        // and before its end.
        //
        // REVISIT: Possible micro-optimisation: `impl Borrow<T> for RangeStartWrapper<T>`
        // and use that to search here, to avoid constructing another `RangeStartWrapper`.
        let new_range_end_as_start = RangeStartWrapper::new(range.end.clone()..range.end.clone());
        while let Some((stored_range_start_wrapper, stored_value)) = self
            .btm
            .range((
                Bound::Excluded(&range_start_wrapper),
                Bound::Excluded(&new_range_end_as_start),
            ))
            .next()
            .map(|(stored_range_start_wrapper, stored_value)| {
                (stored_range_start_wrapper.clone(), stored_value.clone())
            })
        {
            self.adjust_overlapping_ranges_for_remove(
                stored_range_start_wrapper,
                stored_value,
                &range,
            );
        }
    }

    // TODO: Implement remove_entry()? It could just be folded into remove, returning the
    // original range of the items removed... But we get the same overlap issue with insert

    // TODO: retain()? Relies on unsatble drain_filter, though...

    // TODO: check len() in doctest
    /// Moves all elements from `other` into `Self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// let mut a = BTreeMap::new();
    /// a.insert(0..1, "a");
    /// a.insert(1..2, "b");
    /// a.insert(2..3, "c");
    ///
    /// let mut b = BTreeMap::new();
    /// b.insert(2..3, "d");
    /// b.insert(3..4, "e");
    /// b.insert(4..5, "f");
    ///
    /// a.append(&mut b);
    ///
    /// // assert_eq!(a.len(), 5);
    /// // assert_eq!(b.len(), 0);
    ///
    /// assert_eq!(a[&0], "a");
    /// assert_eq!(a[&1], "b");
    /// assert_eq!(a[&2], "d");
    /// assert_eq!(a[&3], "e");
    /// assert_eq!(a[&4], "f");
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        // Do we have to append anything at all?
        if other.is_empty() {
            return;
        }

        // We can just swap `self` and `other` if `self` is empty.
        if self.is_empty() {
            core::mem::swap(self, other);
            return;
        }

        // Otherwise, insert everything from other into self
        // TODO: implementation here is not exactly what BTreeMap does... This is pretty much just .extend()
        let other_iter = core::mem::take(other).into_iter();
        for (range, value) in other_iter {
            self.insert(range, value)
        }
    }

    // TODO: description / Tests
    // This is probably what get_mut should point to
    // /// Gets the given key's corresponding entry in the map for in-place manipulation.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // ///
    // /// ```
    // /// use std::collections::BTreeMap;
    // ///
    // /// let mut count: BTreeMap<&str, usize> = BTreeMap::new();
    // ///
    // /// // count the number of occurrences of letters in the vec
    // /// for x in vec!["a", "b", "a", "c", "a", "b"] {
    // ///     *count.entry(x).or_insert(0) += 1;
    // /// }
    // ///
    // /// assert_eq!(count["a"], 3);
    // /// ```
    // pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
    //     self.get_key_value(&k);
    //     // TODO: Entry api
    //     todo!()
    // }

    // TODO: doctest
    /// Splits the map into two at the given point. Returns everything after the given key,
    /// including the key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// let mut a = BTreeMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// a.insert(3, "c");
    /// a.insert(17, "d");
    /// a.insert(41, "e");
    ///
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    ///
    /// assert_eq!(a[&1], "a");
    /// assert_eq!(a[&2], "b");
    ///
    /// assert_eq!(b[&3], "c");
    /// assert_eq!(b[&17], "d");
    /// assert_eq!(b[&41], "e");
    /// ```
    pub fn split_off<Q: ?Sized + Ord>(&mut self, key: &Q) -> Self
    where
        K: Borrow<Q> + Ord,
    {
        if self.is_empty() {
            return Self::new();
        }

        // TODO: check if this key is in a range and if we need to split one,
        // If it is, we'll need to put that range on each side
        todo!()

        // let total_num = self.len();
        // let left_root = self.root.as_mut().unwrap(); // unwrap succeeds because not empty

        // let right_root = left_root.split_off(key);

        // let (new_left_len, right_len) = Root::calc_split_length(total_num, &left_root, &right_root);
        // self.length = new_left_len;

        // BTreeMap { root: Some(right_root), length: right_len }
    }

    fn adjust_touching_ranges_for_insert(
        &mut self,
        stored_range_start_wrapper: RangeStartWrapper<K>,
        stored_value: V,
        new_range: &mut Range<K>,
        new_value: &V,
    ) {
        use core::cmp::{max, min};

        if stored_value == *new_value {
            // The ranges have the same value, so we can "adopt"
            // the stored range.
            //
            // This means that no matter how big or where the stored range is,
            // we will expand the new range's bounds to subsume it,
            // and then delete the stored range.
            new_range.start =
                min(&new_range.start, &stored_range_start_wrapper.range.start).clone();
            new_range.end = max(&new_range.end, &stored_range_start_wrapper.range.end).clone();
            self.btm.remove(&stored_range_start_wrapper);
        } else {
            // The ranges have different values.
            if new_range.overlaps(&stored_range_start_wrapper.range) {
                // The ranges overlap. This is a little bit more complicated.
                // Delete the stored range, and then add back between
                // 0 and 2 subranges at the ends of the range to insert.
                self.btm.remove(&stored_range_start_wrapper);
                if stored_range_start_wrapper.range.start < new_range.start {
                    // Insert the piece left of the range to insert.
                    self.btm.insert(
                        RangeStartWrapper::new(
                            stored_range_start_wrapper.range.start..new_range.start.clone(),
                        ),
                        stored_value.clone(),
                    );
                }
                if stored_range_start_wrapper.range.end > new_range.end {
                    // Insert the piece right of the range to insert.
                    self.btm.insert(
                        RangeStartWrapper::new(
                            new_range.end.clone()..stored_range_start_wrapper.range.end,
                        ),
                        stored_value,
                    );
                }
            } else {
                // No-op; they're not overlapping,
                // so we can just keep both ranges as they are.
            }
        }
    }

    fn adjust_overlapping_ranges_for_remove(
        &mut self,
        stored_range_start_wrapper: RangeStartWrapper<K>,
        stored_value: V,
        range_to_remove: &Range<K>,
    ) {
        // Delete the stored range, and then add back between
        // 0 and 2 subranges at the ends of the range to insert.
        self.btm.remove(&stored_range_start_wrapper);
        let stored_range = stored_range_start_wrapper.range;
        if stored_range.start < range_to_remove.start {
            // Insert the piece left of the range to insert.
            self.btm.insert(
                RangeStartWrapper::new(stored_range.start..range_to_remove.start.clone()),
                stored_value.clone(),
            );
        }
        if stored_range.end > range_to_remove.end {
            // Insert the piece right of the range to insert.
            self.btm.insert(
                RangeStartWrapper::new(range_to_remove.end.clone()..stored_range.end),
                stored_value,
            );
        }
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the map.
    ///
    /// The iterator element type is `Range<K>`.
    ///
    /// NOTE: Calling `gaps` eagerly finds the first gap,
    /// even if the iterator is never consumed.
    pub fn gaps<'a>(&'a self, outer_range: &'a Range<K>) -> Gaps<'a, K, V> {
        let mut keys = self.btm.keys().peekable();

        // Find the first potential gap.
        let mut candidate_start = &outer_range.start;
        while let Some(item) = keys.peek() {
            if item.range.end <= outer_range.start {
                // This range sits entirely before the start of
                // the outer range; just skip it.
                let _ = keys.next();
            } else if item.range.start <= outer_range.start {
                // This range overlaps the start of the
                // outer range, so the first possible candidate
                // range begins at its end.
                candidate_start = &item.range.end;
                let _ = keys.next();
            } else {
                // The rest of the items might contribute to gaps.
                break;
            }
        }

        Gaps {
            outer_range,
            keys,
            candidate_start,
        }
    }
}

impl<K: Hash, V: Hash> Hash for RangeMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for elt in self {
            elt.hash(state);
        }
    }
}

impl<K, V> Default for RangeMap<K, V>
where
    K: Ord + Clone,
    V: Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for RangeMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
    }
}
impl<K: Eq, V: Eq> Eq for RangeMap<K, V> {}
impl<K: PartialOrd + Ord, V: PartialOrd> PartialOrd for RangeMap<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.btm.iter().partial_cmp(other.btm.iter())
    }
}
impl<K: Ord, V: Ord> Ord for RangeMap<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.btm.iter().cmp(other.btm.iter())
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeMap does.
impl<K: Debug, V: Debug> Debug for RangeMap<K, V>
where
    K: Ord + Clone,
    V: Eq + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> Index<&K> for RangeMap<K, V>
where
    K: Clone + Ord,
    V: Eq + Clone,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `RangeMap`.
    #[inline]
    fn index(&self, key: &K) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

pub struct Gaps<'a, K, V> {
    outer_range: &'a Range<K>,
    keys: core::iter::Peekable<alloc::collections::btree_map::Keys<'a, RangeStartWrapper<K>, V>>,
    candidate_start: &'a K,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, K, V> core::iter::FusedIterator for Gaps<'a, K, V> where K: Ord + Clone {}

impl<'a, K, V> Iterator for Gaps<'a, K, V>
where
    K: Ord + Clone,
{
    type Item = Range<K>;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.candidate_start >= self.outer_range.end {
            // We've already passed the end of the outer range;
            // there are no more gaps to find.
            return None;
        }

        // Figure out where this gap ends.
        let (end, next_candidate_start) = if let Some(item) = self.keys.next() {
            if item.range.start < self.outer_range.end {
                // The gap goes up until the start of the next item,
                // and the next candidate starts after it.
                (&item.range.start, &item.range.end)
            } else {
                // The item sits after the end of the outer range,
                // so this gap ends at the end of the outer range.
                // This also means there will be no more gaps.
                (&self.outer_range.end, &self.outer_range.end)
            }
        } else {
            // There's no next item; the end is at the
            // end of the outer range.
            // This also means there will be no more gaps.
            (&self.outer_range.end, &self.outer_range.end)
        };

        // Move the next candidate gap start past the end
        // of this gap, and yield the gap we found.
        let gap = self.candidate_start.clone()..end.clone();
        self.candidate_start = next_candidate_start;
        Some(gap)
    }
}

pub mod iterators {
    // TODO: add Gaps here
    use core::borrow::Borrow;
    use core::fmt::{self, Debug};
    use core::iter::{FromIterator, FusedIterator};
    use core::ops::{Range, RangeBounds};

    use super::{RangeMap, RangeStartWrapper};

    impl<K, V> RangeMap<K, V> {
        // TODO: doctest
        /// Gets an iterator over the entries of the map, sorted by key.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut map = BTreeMap::new();
        /// map.insert(3, "c");
        /// map.insert(2, "b");
        /// map.insert(1, "a");
        ///
        /// for (key, value) in map.iter() {
        ///     println!("{}: {}", key, value);
        /// }
        ///
        /// let (first_key, first_value) = map.iter().next().unwrap();
        /// assert_eq!((*first_key, *first_value), (1, "a"));
        /// ```
        pub fn iter(&self) -> Iter<'_, K, V> {
            Iter(self.btm.iter())
        }

        // TODO: doctest
        /// Gets a mutable iterator over the entries of the map, sorted by key.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut map = BTreeMap::new();
        /// map.insert("a", 1);
        /// map.insert("b", 2);
        /// map.insert("c", 3);
        ///
        /// // add 10 to the value if the key isn't "a"
        /// for (key, value) in map.iter_mut() {
        ///     if key != &"a" {
        ///         *value += 10;
        ///     }
        /// }
        /// ```
        pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
            IterMut(self.btm.iter_mut())
        }

        // TODO: doctest
        /// Gets an iterator over the keys of the map, in sorted order.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut a = BTreeMap::new();
        /// a.insert(2, "b");
        /// a.insert(1, "a");
        ///
        /// let keys: Vec<_> = a.keys().cloned().collect();
        /// assert_eq!(keys, [1, 2]);
        /// ```
        pub fn keys(&self) -> Keys<'_, K, V> {
            Keys(self.iter())
        }

        // TODO: doctest
        /// Gets an iterator over the values of the map, in order by key.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut a = BTreeMap::new();
        /// a.insert(1, "hello");
        /// a.insert(2, "goodbye");
        ///
        /// let values: Vec<&str> = a.values().cloned().collect();
        /// assert_eq!(values, ["hello", "goodbye"]);
        /// ```
        pub fn values(&self) -> Values<'_, K, V> {
            Values(self.iter())
        }

        // TODO: doctest
        /// Gets a mutable iterator over the values of the map, in order by key.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut a = BTreeMap::new();
        /// a.insert(1, String::from("hello"));
        /// a.insert(2, String::from("goodbye"));
        ///
        /// for value in a.values_mut() {
        ///     value.push_str("!");
        /// }
        ///
        /// let values: Vec<String> = a.values().cloned().collect();
        /// assert_eq!(values, [String::from("hello!"),
        ///                     String::from("goodbye!")]);
        /// ```
        pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
            ValuesMut(self.iter_mut())
        }

        // TODO: doctest
        /// Returns the number of elements in the map.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut a = BTreeMap::new();
        /// assert_eq!(a.len(), 0);
        /// a.insert(1, "a");
        /// assert_eq!(a.len(), 1);
        /// ```
        pub const fn len(&self) -> usize {
            self.btm.len()
        }

        // TODO: doctest
        /// Returns `true` if the map contains no elements.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use std::collections::BTreeMap;
        ///
        /// let mut a = BTreeMap::new();
        /// assert!(a.is_empty());
        /// a.insert(1, "a");
        /// assert!(!a.is_empty());
        /// ```
        pub const fn is_empty(&self) -> bool {
            self.btm.is_empty()
        }

        // /// Constructs a double-ended iterator over a sub-range of elements in the map.
        // /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
        // /// yield elements from min (inclusive) to max (exclusive).
        // /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
        // /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
        // /// range from 4 to 10.
        // ///
        // /// # Panics
        // ///
        // /// Panics if range `start > end`.
        // /// Panics if range `start == end` and both bounds are `Excluded`.
        // ///
        // /// # Examples
        // ///
        // /// Basic usage:
        // ///
        // /// ```
        // /// use std::collections::BTreeMap;
        // /// use std::ops::Bound::Included;
        // ///
        // /// let mut map = BTreeMap::new();
        // /// map.insert(3, "a");
        // /// map.insert(5, "b");
        // /// map.insert(8, "c");
        // /// for (&key, &value) in map.range((Included(&4), Included(&8))) {
        // ///     println!("{}: {}", key, value);
        // /// }
        // /// assert_eq!(Some((&5, &"b")), map.range(4..).next());
        // /// ```
        // pub fn range<T: ?Sized, R>(&self, range: R) -> XRange<'_, K, V>
        // where
        //     T: Ord,
        //     K: Borrow<T> + Ord,
        //     R: RangeBounds<T>,
        // {
        //     // TODO
        //     todo!()
        // }

        // /// Constructs a mutable double-ended iterator over a sub-range of elements in the map.
        // /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
        // /// yield elements from min (inclusive) to max (exclusive).
        // /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
        // /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
        // /// range from 4 to 10.
        // ///
        // /// # Panics
        // ///
        // /// Panics if range `start > end`.
        // /// Panics if range `start == end` and both bounds are `Excluded`.
        // ///
        // /// # Examples
        // ///
        // /// Basic usage:
        // ///
        // /// ```
        // /// use std::collections::BTreeMap;
        // ///
        // /// let mut map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
        // ///     .iter()
        // ///     .map(|&s| (s, 0))
        // ///     .collect();
        // /// for (_, balance) in map.range_mut("B".."Cheryl") {
        // ///     *balance += 100;
        // /// }
        // /// for (name, balance) in &map {
        // ///     println!("{} => {}", name, balance);
        // /// }
        // /// ```
        // pub fn range_mut<T: ?Sized, R>(&mut self, range: R) -> XRangeMut<'_, K, V>
        // where
        //     T: Ord,
        //     K: Borrow<T> + Ord,
        //     R: RangeBounds<T>,
        // {
        //     todo!()
        //     // TODO
        // }
    }

    impl<'a, K, V> IntoIterator for &'a RangeMap<K, V> {
        type Item = (&'a Range<K>, &'a V);
        type IntoIter = Iter<'a, K, V>;
        fn into_iter(self) -> Iter<'a, K, V> {
            self.iter()
        }
    }

    /// An iterator over the entries of a `RangeMap`.
    ///
    /// This `struct` is created by the [`iter`] method on [`RangeMap`]. See its
    /// documentation for more.
    ///
    /// [`iter`]: RangeMap::iter
    pub struct Iter<'a, K, V>(alloc::collections::btree_map::Iter<'a, RangeStartWrapper<K>, V>);
    impl<K: Debug, V: Debug> Debug for Iter<'_, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone()).finish()
        }
    }
    impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
        type Item = (&'a Range<K>, &'a V);
        fn next(&mut self) -> Option<(&'a Range<K>, &'a V)> {
            self.0.next().map(|(wrapper, v)| (&wrapper.range, v))
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
        fn last(mut self) -> Option<(&'a Range<K>, &'a V)> {
            self.next_back()
        }
        fn min(mut self) -> Option<(&'a Range<K>, &'a V)> {
            self.next()
        }
        fn max(mut self) -> Option<(&'a Range<K>, &'a V)> {
            self.next_back()
        }
    }
    impl<K, V> FusedIterator for Iter<'_, K, V> {}

    impl<'a, K: 'a, V: 'a> DoubleEndedIterator for Iter<'a, K, V> {
        fn next_back(&mut self) -> Option<(&'a Range<K>, &'a V)> {
            self.0.next_back().map(|(wrapper, v)| (&wrapper.range, v))
        }
    }
    impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<K, V> Clone for Iter<'_, K, V> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<'a, K, V> IntoIterator for &'a mut RangeMap<K, V> {
        type Item = (&'a Range<K>, &'a mut V);
        type IntoIter = IterMut<'a, K, V>;
        fn into_iter(self) -> IterMut<'a, K, V> {
            self.iter_mut()
        }
    }
    /// A mutable iterator over the entries of a `RangeMap`.
    ///
    /// This `struct` is created by the [`iter_mut`] method on [`RangeMap`]. See its
    /// documentation for more.
    ///
    /// [`iter_mut`]: RangeMap::iter_mut
    pub struct IterMut<'a, K: 'a, V: 'a>(
        alloc::collections::btree_map::IterMut<'a, RangeStartWrapper<K>, V>,
    );
    // TODO
    impl<K: Debug, V: Debug> Debug for IterMut<'_, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<'a, K: 'a, V: 'a> Iterator for IterMut<'a, K, V> {
        type Item = (&'a Range<K>, &'a mut V);

        fn next(&mut self) -> Option<(&'a Range<K>, &'a mut V)> {
            self.0.next().map(|(wrapper, v)| (&wrapper.range, v))
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }

        fn last(mut self) -> Option<(&'a Range<K>, &'a mut V)> {
            self.next_back()
        }

        fn min(mut self) -> Option<(&'a Range<K>, &'a mut V)> {
            self.next()
        }

        fn max(mut self) -> Option<(&'a Range<K>, &'a mut V)> {
            self.next_back()
        }
    }

    impl<'a, K: 'a, V: 'a> DoubleEndedIterator for IterMut<'a, K, V> {
        fn next_back(&mut self) -> Option<(&'a Range<K>, &'a mut V)> {
            self.0.next_back().map(|(wrapper, v)| (&wrapper.range, v))
        }
    }

    impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl<K, V> FusedIterator for IterMut<'_, K, V> {}

    // impl<'a, K, V> IterMut<'a, K, V> {
    //     /// Returns an iterator of references over the remaining items.
    //     #[inline]
    //     pub(super) fn iter(&self) -> Iter<'_, K, V> {
    //         Iter(self.0.iter())
    //     }
    // }

    // TODO: Move to Iterators
    impl<K, V> IntoIterator for RangeMap<K, V> {
        type Item = (Range<K>, V);
        type IntoIter = IntoIter<K, V>;
        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self.btm.into_iter())
        }
    }
    /// An owning iterator over the entries of a `RangeMap`.
    ///
    /// This `struct` is created by the [`into_iter`] method on [`RangeMap`]
    /// (provided by the `IntoIterator` trait). See its documentation for more.
    ///
    /// [`into_iter`]: IntoIterator::into_iter
    pub struct IntoIter<K, V>(alloc::collections::btree_map::IntoIter<RangeStartWrapper<K>, V>);
    // impl<K: Debug, V: Debug> Debug for IntoIter<K, V> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_list().entries(self.iter()).finish()
    //     }
    // }
    impl<K, V> Iterator for IntoIter<K, V> {
        type Item = (Range<K>, V);
        fn next(&mut self) -> Option<(Range<K>, V)> {
            self.0.next().map(|(wrapper, v)| (wrapper.range, v))
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
    impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
        fn next_back(&mut self) -> Option<(Range<K>, V)> {
            self.0.next_back().map(|(wrapper, v)| (wrapper.range, v))
        }
    }
    impl<K, V> ExactSizeIterator for IntoIter<K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<K, V> FusedIterator for IntoIter<K, V> {}
    // impl<K, V> IntoIter<K, V> {
    //     #[inline]
    //     pub(super) fn iter(&self) -> Iter<'_, K, V> {
    //         Iter(self.)
    //     }
    // }

    /// An iterator over the keys of a `RangeMap`.
    ///
    /// This `struct` is created by the [`keys`] method on [`RangeMap`]. See its
    /// documentation for more.
    ///
    /// [`keys`]: RangeMap::keys
    #[derive(Clone)]
    pub struct Keys<'a, K: 'a, V: 'a>(Iter<'a, K, V>);
    // TODO
    // impl<K: Debug, V> Debug for Keys<'_, K, V> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_list().entries(self.clone()).finish()
    //     }
    // }
    impl<'a, K, V> Iterator for Keys<'a, K, V> {
        type Item = &'a Range<K>;
        fn next(&mut self) -> Option<&'a Range<K>> {
            self.0.next().map(|(k, _)| k)
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
        fn last(mut self) -> Option<&'a Range<K>> {
            self.next_back()
        }
        fn min(mut self) -> Option<&'a Range<K>> {
            self.next()
        }
        fn max(mut self) -> Option<&'a Range<K>> {
            self.next_back()
        }
    }
    impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
        fn next_back(&mut self) -> Option<&'a Range<K>> {
            self.0.next_back().map(|(k, _)| k)
        }
    }
    impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<K, V> FusedIterator for Keys<'_, K, V> {}

    /// An iterator over the values of a `RangeMap`.
    ///
    /// This `struct` is created by the [`values`] method on [`RangeMap`]. See its
    /// documentation for more.
    ///
    /// [`values`]: RangeMap::values
    #[derive(Clone)]
    pub struct Values<'a, K: 'a, V: 'a>(Iter<'a, K, V>);
    // TODO
    // impl<K, V: Debug> Debug for Values<'_, K, V> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_list().entries(self.clone()).finish()
    //     }
    // }
    impl<'a, K, V> Iterator for Values<'a, K, V> {
        type Item = &'a V;
        fn next(&mut self) -> Option<&'a V> {
            self.0.next().map(|(_, v)| v)
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
        fn last(mut self) -> Option<&'a V> {
            self.next_back()
        }
    }
    impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
        fn next_back(&mut self) -> Option<&'a V> {
            self.0.next_back().map(|(_, v)| v)
        }
    }
    impl<K, V> ExactSizeIterator for Values<'_, K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<K, V> FusedIterator for Values<'_, K, V> {}

    /// A mutable iterator over the values of a `RangeMap`.
    ///
    /// This `struct` is created by the [`values_mut`] method on [`RangeMap`]. See its
    /// documentation for more.
    ///
    /// [`values_mut`]: RangeMap::values_mut
    pub struct ValuesMut<'a, K: 'a, V: 'a>(IterMut<'a, K, V>);
    // TODO
    // impl<K, V: Debug> Debug for ValuesMut<'_, K, V> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_list()
    //             .entries(self.iter().map(|(_, val)| val))
    //             .finish()
    //     }
    // }
    impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
        type Item = &'a mut V;
        fn next(&mut self) -> Option<&'a mut V> {
            self.0.next().map(|(_, v)| v)
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
        fn last(mut self) -> Option<&'a mut V> {
            self.next_back()
        }
    }
    impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
        fn next_back(&mut self) -> Option<&'a mut V> {
            self.0.next_back().map(|(_, v)| v)
        }
    }
    impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}

    // TODO: method name?
    // TODO: struct names
    // /// An iterator over a sub-range of entries in a `RangeMap`.
    // ///
    // /// This `struct` is created by the [`range`] method on [`RangeMap`]. See its
    // /// documentation for more.
    // ///
    // /// [`range`]: RangeMap::range
    // pub struct XRange<'a, K: 'a, V: 'a>(alloc::collections::btree_map::Range<'a, K, V>);
    // impl<K: Debug, V: Debug> Debug for XRange<'_, K, V> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_list().entries(self.clone()).finish()
    //     }
    // }
    // impl<'a, K, V> Iterator for XRange<'a, K, V> {
    //     type Item = (&'a K, &'a V);

    //     fn next(&mut self) -> Option<(&'a K, &'a V)> {
    //         if self.inner.is_empty() {
    //             None
    //         } else {
    //             Some(unsafe { self.next_unchecked() })
    //         }
    //     }

    //     fn last(mut self) -> Option<(&'a K, &'a V)> {
    //         self.next_back()
    //     }

    //     fn min(mut self) -> Option<(&'a K, &'a V)> {
    //         self.next()
    //     }

    //     fn max(mut self) -> Option<(&'a K, &'a V)> {
    //         self.next_back()
    //     }
    // }
    // impl<'a, K, V> DoubleEndedIterator for XRange<'a, K, V> {
    //     fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
    //         if self.inner.is_empty() {
    //             None
    //         } else {
    //             Some(unsafe { self.next_back_unchecked() })
    //         }
    //     }
    // }
    // impl<K, V> FusedIterator for Range<'_, K, V> {}

    // /// A mutable iterator over a sub-range of entries in a `RangeMap`.
    // ///
    // /// This `struct` is created by the [`range_mut`] method on [`RangeMap`]. See its
    // /// documentation for more.
    // ///
    // /// [`range_mut`]: RangeMap::range_mut
    // pub struct XRangeMut<'a, K: 'a, V: 'a>(alloc::collections::btree_map::RangeMut<'a, K, V>);
    // // impl<K: Debug, V: Debug> Debug for RangeMut<'_, K, V> {
    // //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // //         let range = Range { inner: self.inner.reborrow() };
    // //         f.debug_list().entries(range).finish()
    // //     }
    // // }
    // impl<'a, K, V> Iterator for RangeMut<'a, K, V> {
    //     type Item = (&'a K, &'a mut V);

    //     fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
    //         if self.inner.is_empty() {
    //             None
    //         } else {
    //             Some(unsafe { self.next_unchecked() })
    //         }
    //     }

    //     fn last(mut self) -> Option<(&'a K, &'a mut V)> {
    //         self.next_back()
    //     }

    //     fn min(mut self) -> Option<(&'a K, &'a mut V)> {
    //         self.next()
    //     }

    //     fn max(mut self) -> Option<(&'a K, &'a mut V)> {
    //         self.next_back()
    //     }
    // }
    // impl<'a, K, V> DoubleEndedIterator for RangeMut<'a, K, V> {
    //     fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
    //         if self.inner.is_empty() {
    //             None
    //         } else {
    //             Some(unsafe { self.next_back_unchecked() })
    //         }
    //     }
    // }
    // impl<K, V> FusedIterator for RangeMut<'_, K, V> {}

    // // Support collecting
    // impl<K: Ord, V> FromIterator<(Range<K>, V)> for RangeMap<K, V> {
    //     fn from_iter<T: IntoIterator<Item = (Range<K>, V)>>(iter: T) -> RangeMap<K, V> {
    //         let mut map = RangeMap::new();
    //         map.extend(iter);
    //         map
    //     }
    // }
    // impl<K: Ord, V> Extend<(K, V)> for RangeMap<K, V> {
    //     #[inline]
    //     fn extend<T: IntoIterator<Item = (Range<K>, V)>>(&mut self, iter: T) {
    //         iter.into_iter().for_each(move |(k, v)| {
    //             self.insert(k, v);
    //         });
    //     }

    //     // #[inline]
    //     // fn extend_one(&mut self, (k, v): (Range<K>, V)) {
    //     //     self.insert(k, v);
    //     // }
    // }
    // impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a Range<K>, &'a V)> for RangeMap<K, V> {
    //     fn extend<I: IntoIterator<Item = (&'a Range<K>, &'a V)>>(&mut self, iter: I) {
    //         self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    //     }

    //     // #[inline]
    //     // fn extend_one(&mut self, (&k, &v): (&'a K, &'a V)) {
    //     //     self.insert(k, v);
    //     // }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, vec, vec::Vec};

    trait RangeMapExt<K, V> {
        fn to_vec(&self) -> Vec<(Range<K>, V)>;
    }

    impl<K, V> RangeMapExt<K, V> for RangeMap<K, V>
    where
        K: Ord + Clone,
        V: Eq + Clone,
    {
        fn to_vec(&self) -> Vec<(Range<K>, V)> {
            self.iter().map(|(kr, v)| (kr.clone(), v.clone())).collect()
        }
    }

    //
    // Insertion tests
    //

    #[test]
    fn empty_map_is_empty() {
        let range_map: RangeMap<u32, bool> = RangeMap::new();
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.to_vec(), vec![(0..50, false)]);
    }

    #[test]
    fn new_same_value_immediately_following_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_immediately_following_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_immediately_preceding_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_immediately_preceding_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_wholly_inside_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_map.insert(1..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_wholly_inside_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------◇ ◌ ◌ ◌ ◌
        range_map.insert(1..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
        assert_eq!(
            range_map.to_vec(),
            vec![(1..2, true), (2..4, false), (4..5, true)]
        );
    }

    #[test]
    fn replace_at_end_of_existing_range_should_coalesce() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    // Test every permutation of a bunch of touching and overlapping ranges.
    fn lots_of_interesting_ranges() {
        use crate::stupid_range_map::StupidU32RangeMap;
        use permutator::Permutation;

        let mut ranges_with_values = [
            (2..3, false),
            // A duplicate duplicates
            (2..3, false),
            // Almost a duplicate, but with a different value
            (2..3, true),
            // A few small ranges, some of them overlapping others,
            // some of them touching others
            (3..5, true),
            (4..6, true),
            (5..7, true),
            // A really big range
            (2..6, true),
        ];

        ranges_with_values.permutation().for_each(|permutation| {
            let mut range_map: RangeMap<u32, bool> = RangeMap::new();
            let mut stupid: StupidU32RangeMap<bool> = StupidU32RangeMap::new();

            for (k, v) in permutation {
                // Insert it into both maps.
                range_map.insert(k.clone(), v);
                // NOTE: Clippy's `range_minus_one` lint is a bit overzealous here,
                // because we _can't_ pass an open-ended range to `insert`.
                #[allow(clippy::range_minus_one)]
                stupid.insert(k.start..=(k.end - 1), v);

                // At every step, both maps should contain the same stuff.
                let stupid2: StupidU32RangeMap<bool> = range_map.clone().into();
                assert_eq!(stupid, stupid2);
            }
        });
    }

    //
    // Get* tests
    //

    #[test]
    fn get() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.get(&49), Some(&false));
        assert_eq!(range_map.get(&50), None);
    }

    #[test]
    fn get_key_value() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.get_key_value(&49), Some((&(0..50), &false)));
        assert_eq!(range_map.get_key_value(&50), None);
    }

    //
    // Removal tests
    //

    #[test]
    fn remove_from_empty_map() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.remove(0..50);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn remove_non_covered_range_before_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..25);
        assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
    }

    #[test]
    fn remove_non_covered_range_after_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(75..100);
        assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
    }

    #[test]
    fn remove_overlapping_start_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..30);
        assert_eq!(range_map.to_vec(), vec![(30..75, false)]);
    }

    #[test]
    fn remove_middle_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(30..70);
        assert_eq!(range_map.to_vec(), vec![(25..30, false), (70..75, false)]);
    }

    #[test]
    fn remove_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(70..100);
        assert_eq!(range_map.to_vec(), vec![(25..70, false)]);
    }

    #[test]
    fn remove_exactly_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(25..75);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn remove_superset_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..100);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    // Gaps tests

    #[test]
    fn whole_range_is_a_gap() {
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        let range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn whole_range_is_covered_exactly() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_map.insert(1..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◇ ◌ ◌ ◌
        let outer_range = 1..6;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_before_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_touching_start_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_map.insert(1..5, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_overlapping_start_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_map.insert(1..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
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
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_map.insert(5..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield from the item onwards.
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn items_floating_inside_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_map.insert(5..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_map.gaps(&outer_range);
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
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ●-◌ ◌
        range_map.insert(7..8, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
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
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●---◌ ◌ ◌ ◌
        range_map.insert(4..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌
        let outer_range = 2..5;
        let mut gaps = range_map.gaps(&outer_range);
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
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●-------◌ ◌
        range_map.insert(4..8, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_after_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ●---◌ ◌
        range_map.insert(6..7, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_away_from_both_sides() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
        range_map.insert(5..7, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_touching_both_sides() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌
        range_map.insert(4..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_item_straddling() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..5, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    ///
    /// impl Debug
    ///

    #[test]
    fn map_debug_repr_looks_right() {
        let mut map: RangeMap<u32, ()> = RangeMap::new();

        // Empty
        assert_eq!(format!("{:?}", map), "{}");

        // One entry
        map.insert(2..5, ());
        assert_eq!(format!("{:?}", map), "{2..5: ()}");

        // Many entries
        map.insert(6..7, ());
        map.insert(8..9, ());
        assert_eq!(format!("{:?}", map), "{2..5: (), 6..7: (), 8..9: ()}");
    }

    // Iterator Tests

    // TODO: more iterator tests

    // TODO: uncomment
    // #[test]
    // fn into_iter_matches_iter() {
    //     // Just use vec since that's the same implementation we'd expect
    //     let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    //     range_map.insert(1..3, false);
    //     range_map.insert(3..5, true);

    //     let cloned = range_map.to_vec();
    //     let consumed = range_map.into_iter().collect::<Vec<_>>();

    //     // Correct value
    //     assert_eq!(cloned, vec![(1..3, false), (3..5, true)]);

    //     // Equality
    //     assert_eq!(cloned, consumed);
    // }
}
