use num::Bounded;
use std::ops::{Range, RangeInclusive};

pub trait RangeExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
    // TODO: Remove once https://github.com/rust-lang/rust/issues/32311
    // is stabilized.
    fn contains_item(&self, item: &T) -> bool;
}

impl<T> RangeExt<T> for Range<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        use std::cmp::{max, min};
        // Strictly less than, because ends are excluded.
        max(&self.start, &other.start) < min(&self.end, &other.end)
    }

    fn touches(&self, other: &Self) -> bool {
        use std::cmp::{max, min};
        // Less-than-or-equal-to because if one end is excluded, the other is included.
        // I.e. the two could be joined into a single range, because they're overlapping
        // or immediately adjacent.
        max(&self.start, &other.start) <= min(&self.end, &other.end)
    }

    // TODO: Remove once https://github.com/rust-lang/rust/issues/32311
    // is stabilized.
    fn contains_item(&self, item: &T) -> bool {
        *item >= self.start && *item < self.end
    }
}

pub trait RangeInclusiveExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool
    where
        T: StepLite + Bounded + Clone;
    // TODO: Remove once https://github.com/rust-lang/rust/issues/32311
    // is stabilized.
    fn contains_item(&self, item: &T) -> bool;
}

impl<T> RangeInclusiveExt<T> for RangeInclusive<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        use std::cmp::{max, min};
        // Less than or equal, because ends are included.
        max(self.start(), other.start()) <= min(self.end(), other.end())
    }

    fn touches(&self, other: &Self) -> bool
    where
        T: StepLite + Bounded + Clone,
    {
        use std::cmp::{max, min};
        // Touching for end-inclusive ranges is equivalent to touching of
        // slightly longer end-inclusive ranges.
        //
        // We need to do this dance to avoid arithmetic overflow
        // at the extremes of the key space.
        let longer_self_end: T = if *self.end() == T::max_value() {
            (*self.end()).clone()
        } else {
            self.end().add_one()
        };
        let longer_other_end: T = if *other.end() == T::max_value() {
            other.end().clone()
        } else {
            other.end().add_one()
        };
        max(self.start(), other.start()) <= min(&longer_self_end, &longer_other_end)
    }

    // TODO: Remove once https://github.com/rust-lang/rust/issues/32311
    // is stabilized.
    fn contains_item(&self, item: &T) -> bool {
        *item >= *self.start() && *item <= *self.end()
    }
}

/// Minimal version of unstable [Step](std::iter::Step) trait
/// from the Rust standard library.
///
/// This is needed for [RangeInclusiveMap](crate::RangeInclusiveMap)
/// because ranges stored as its keys interact with each other
/// when the start of one is _adjacent_ the end of another.
/// I.e. we need a concept of successor values rather than just
/// equality, and that is what [Step](std::iter::Step) will
/// eventually provide once it is stabilized.
//
// TODO: Deprecate and then eventually remove once
// https://github.com/rust-lang/rust/issues/42168 is stabilized.
pub trait StepLite {
    fn add_one(&self) -> Self;
    fn sub_one(&self) -> Self;
}

// Makes tests work.
impl StepLite for u32 {
    fn add_one(&self) -> Self {
        self + 1
    }

    fn sub_one(&self) -> Self {
        self - 1
    }
}

impl StepLite for u8 {
    fn add_one(&self) -> Self {
        self + 1
    }

    fn sub_one(&self) -> Self {
        self - 1
    }
}

// TODO: Auto-implement for all common types?
// But people will still need to implement it for
// their own types, so need to find the least painful
// way for consumers.

// TODO: When on nightly, a blanket implementation for
// all types that implement `std::iter::Step`.
