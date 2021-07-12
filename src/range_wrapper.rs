// Wrappers to allow storing (and sorting/searching)
// ranges as the keys of a `BTreeMap`.
//
// We can do this because we maintain the invariants
// that the order of range starts is the same as the order
// of range ends, and that no two stored ranges have the
// same start or end as each other.
//
// NOTE: Be very careful not to accidentally use these
// if you really do want to compare equality of the
// inner range!

use core::cmp::Ordering;
use core::ops::{Range, RangeInclusive};

//
// Range start wrapper
//

#[derive(Eq, Debug, Clone)]
pub struct RangeStartWrapper<T> {
    pub range: Range<T>,
}

impl<T> RangeStartWrapper<T> {
    pub fn new(range: Range<T>) -> RangeStartWrapper<T> {
        RangeStartWrapper { range }
    }
    pub fn point(at: T) -> RangeStartWrapper<T> where T:Clone {
        RangeStartWrapper {range: at.clone()..at }
    }
}

impl<T> PartialEq for RangeStartWrapper<T>
where
    T: Eq,
{
    fn eq(&self, other: &RangeStartWrapper<T>) -> bool {
        self.range.start == other.range.start
    }
}

impl<T> Ord for RangeStartWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeStartWrapper<T>) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl<T> PartialOrd for RangeStartWrapper<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &RangeStartWrapper<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//
// RangeInclusive start wrapper
//

#[derive(Eq, Debug, Clone)]
pub struct RangeInclusiveStartWrapper<T> {
    pub range: RangeInclusive<T>,
}

impl<T> RangeInclusiveStartWrapper<T> {
    pub fn new(range: RangeInclusive<T>) -> RangeInclusiveStartWrapper<T> {
        RangeInclusiveStartWrapper { range }
    }
}

impl<T> PartialEq for RangeInclusiveStartWrapper<T>
where
    T: Eq,
{
    fn eq(&self, other: &RangeInclusiveStartWrapper<T>) -> bool {
        self.range.start() == other.range.start()
    }
}

impl<T> Ord for RangeInclusiveStartWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeInclusiveStartWrapper<T>) -> Ordering {
        self.range.start().cmp(&other.range.start())
    }
}

impl<T> PartialOrd for RangeInclusiveStartWrapper<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &RangeInclusiveStartWrapper<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
