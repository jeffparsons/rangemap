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
    pub fn len(&self) -> usize {
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
    pub fn is_empty(&self) -> bool {
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

impl<K, V> RangeMap<K, V>
where
    K: Ord + Clone,
    V: Eq + Clone,
{
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
