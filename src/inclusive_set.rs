use std::ops::RangeInclusive;

use crate::std_ext::*;
use crate::RangeInclusiveMap;

#[derive(Clone)]
/// A set whose items are stored as ranges bounded
/// inclusively below and above `(start..=end)`.
///
/// See [`RangeInclusiveMap`]'s documentation for more details.
///
/// [`RangeInclusiveMap`]: struct.RangeInclusiveMap.html
pub struct RangeInclusiveSet<T, StepFnsT = T> {
    rm: RangeInclusiveMap<T, (), StepFnsT>,
}

impl<T> Default for RangeInclusiveSet<T, T>
where
    T: Ord + Clone + StepLite,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RangeInclusiveSet<T, T>
where
    T: Ord + Clone + StepLite,
{
    /// Makes a new empty `RangeInclusiveSet`.
    pub fn new() -> Self {
        Self::new_with_step_fns()
    }
}

impl<T, StepFnsT> RangeInclusiveSet<T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
    /// Makes a new empty `RangeInclusiveSet`, specifying successor and
    /// predecessor functions defined separately from `T` itself.
    ///
    /// This is useful as a workaround for Rust's "orphan rules",
    /// which prevent you from implementing `StepLite` for `T` if `T`
    /// is a foreign type.
    ///
    /// **NOTE:** This will likely be deprecated and then eventually
    /// removed once the standard library's [Step](std::iter::Step)
    /// trait is stabilised, as most crates will then likely implement [Step](std::iter::Step)
    /// for their types where appropriate.
    ///
    /// See [this issue](https://github.com/rust-lang/rust/issues/42168)
    /// for details about that stabilization process.
    pub fn new_with_step_fns() -> Self {
        Self {
            rm: RangeInclusiveMap::new_with_step_fns(),
        }
    }

    /// Returns a reference to the range covering the given key, if any.
    pub fn get(&self, value: &T) -> Option<&RangeInclusive<T>> {
        self.rm.get_key_value(value).map(|(range, _)| range)
    }

    /// Returns `true` if any range in the set covers the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.rm.contains_key(value)
    }

    /// Gets an ordered iterator over all ranges,
    /// ordered by range.
    pub fn iter(&self) -> impl Iterator<Item = &RangeInclusive<T>> {
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
    /// Panics if range `start > end`.
    pub fn insert(&mut self, range: RangeInclusive<T>) {
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
    /// Panics if range `start > end`.
    pub fn remove(&mut self, range: RangeInclusive<T>) {
        self.rm.remove(range);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait RangeInclusiveSetExt<T> {
        fn to_vec(&self) -> Vec<RangeInclusive<T>>;
    }

    impl<T> RangeInclusiveSetExt<T> for RangeInclusiveSet<T>
    where
        T: Ord + Clone + StepLite,
    {
        fn to_vec(&self) -> Vec<RangeInclusive<T>> {
            self.iter().cloned().collect()
        }
    }

    #[test]
    fn empty_set_is_empty() {
        let range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        assert_eq!(range_set.to_vec(), vec![0..=50]);
    }

    #[test]
    fn remove_partially_overlapping() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        range_set.remove(25..=75);
        assert_eq!(range_set.to_vec(), vec![0..=24]);
    }
}
