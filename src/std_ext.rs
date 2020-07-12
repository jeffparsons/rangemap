use std::ops::{Range, RangeInclusive};

pub trait RangeExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
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
}

pub trait RangeInclusiveExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt<T> for RangeInclusive<T>
where
    T: Ord + StepLite + Clone,
{
    fn overlaps(&self, other: &Self) -> bool {
        use std::cmp::{max, min};
        // Less than or equal, because ends are included.
        max(self.start(), other.start()) <= min(self.end(), other.end())
    }

    fn touches(&self, other: &Self) -> bool {
        use std::cmp::{max, min};

        // Touching for end-inclusive ranges is equivalent to touching of
        // slightly longer end-inclusive ranges.
        //
        // We need to do a small dance to avoid arithmetic overflow
        // at the extremes of the key space. And to do this without
        // needing to bound our key type on something like `num::Bounded`
        // (https://docs.rs/num/0.3.0/num/trait.Bounded.html),
        // we'll just extend the end of the _earlier_ range iff
        // its end is already earlier than the latter range's start.
        let max_start = max(self.start(), other.start());
        let min_range_end = min(self.end(), other.end());
        let min_range_end_extended = if min_range_end < max_start {
            min_range_end.add_one()
        } else {
            min_range_end.clone()
        };
        *max_start <= min_range_end_extended
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
