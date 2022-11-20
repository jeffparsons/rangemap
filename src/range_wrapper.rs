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
use core::ops::RangeInclusive;

use crate::RangeTrait;

//
// Range start wrapper
//

#[derive(Debug, Clone)]
pub struct RangeStartWrapper<T> {
    pub range: T,
}

impl<T> RangeStartWrapper<T> {
    pub fn new(range: T) -> RangeStartWrapper<T> {
        RangeStartWrapper { range }
    }
}

impl<T, I> PartialEq for RangeStartWrapper<T>
where
    T: RangeTrait<A = I>,
    I: PartialEq,
{
    fn eq(&self, other: &RangeStartWrapper<T>) -> bool {
        self.range.start() == other.range.start()
    }
}

impl<T, I> Eq for RangeStartWrapper<T>
where
    T: RangeTrait<A = I>,
    I: Eq,
{
}

impl<T, I> Ord for RangeStartWrapper<T>
where
    T: RangeTrait<A = I>,
    I: Ord,
{
    fn cmp(&self, other: &RangeStartWrapper<T>) -> Ordering {
        self.range.start().cmp(&other.range.start())
    }
}

impl<T, I> PartialOrd for RangeStartWrapper<T>
where
    T: RangeTrait<A = I>,
    I: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeStartWrapper<T>) -> Option<Ordering> {
        self.range.start().partial_cmp(&other.range.start())
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
    T: PartialEq,
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
        self.range.start().cmp(other.range.start())
    }
}

impl<T> PartialOrd for RangeInclusiveStartWrapper<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeInclusiveStartWrapper<T>) -> Option<Ordering> {
        self.range.start().partial_cmp(other.range.start())
    }
}
