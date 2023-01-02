// Wrappers to allow storing (and sorting/searching)
// ranges as the keys of a `BTreeMap`.
//
// This wraps the range in two layers: one that lets us
// order ranges by their start (`RangeStartWrapper`),
// and then within that, one that lets us order them by
// their end (`RangeEndWrapper`). Ordinarily we'll use
// the former, but there are a couple of cases where we
// want to be able to do the latter for performance/convenience.
//
// This is made possible by a sneaky `Borrow` implementation
// which skirts the law about the borrowed representation
// having identical implementations of `Ord` etc., but shouldn't
// be a problem in practice because users of the crate can't
// access these special wrappers, and we are careful to uphold
// invariants that prevent observing any states where the
// differing implementations would produce different results.
//
// Specifically, we maintain the invariants
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

#[derive(Debug, Clone)]
pub struct RangeStartWrapper<T> {
    pub range_end_wrapper: RangeEndWrapper<T>,
}

impl<T> RangeStartWrapper<T> {
    pub fn new(range: Range<T>) -> RangeStartWrapper<T> {
        RangeStartWrapper {
            range_end_wrapper: RangeEndWrapper::new(range),
        }
    }
}

impl<T> PartialEq for RangeStartWrapper<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RangeStartWrapper<T>) -> bool {
        self.range_end_wrapper.range.start == other.range_end_wrapper.range.start
    }
}

impl<T> Eq for RangeStartWrapper<T> where T: Eq {}

impl<T> Ord for RangeStartWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeStartWrapper<T>) -> Ordering {
        self.range_end_wrapper
            .range
            .start
            .cmp(&other.range_end_wrapper.range.start)
    }
}

impl<T> PartialOrd for RangeStartWrapper<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeStartWrapper<T>) -> Option<Ordering> {
        self.range_end_wrapper
            .range
            .start
            .partial_cmp(&other.range_end_wrapper.range.start)
    }
}

impl<T> core::borrow::Borrow<RangeEndWrapper<T>> for RangeStartWrapper<T> {
    fn borrow(&self) -> &RangeEndWrapper<T> {
        &self.range_end_wrapper
    }
}

//
// Range end wrapper
//

#[derive(Debug, Clone)]
pub struct RangeEndWrapper<T> {
    pub range: Range<T>,
}

impl<T> RangeEndWrapper<T> {
    pub fn new(range: Range<T>) -> RangeEndWrapper<T> {
        RangeEndWrapper { range }
    }
}

impl<T> PartialEq for RangeEndWrapper<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RangeEndWrapper<T>) -> bool {
        self.range.end == other.range.end
    }
}

impl<T> Eq for RangeEndWrapper<T> where T: Eq {}

impl<T> Ord for RangeEndWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeEndWrapper<T>) -> Ordering {
        self.range.end.cmp(&other.range.end)
    }
}

impl<T> PartialOrd for RangeEndWrapper<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeEndWrapper<T>) -> Option<Ordering> {
        self.range.end.partial_cmp(&other.range.end)
    }
}

impl<T> core::borrow::Borrow<RangeInclusiveEndWrapper<T>> for RangeInclusiveStartWrapper<T> {
    fn borrow(&self) -> &RangeInclusiveEndWrapper<T> {
        &self.end_wrapper
    }
}

//
// RangeInclusive start wrapper
//

#[derive(Eq, Debug, Clone)]
pub struct RangeInclusiveStartWrapper<T> {
    pub end_wrapper: RangeInclusiveEndWrapper<T>,
}

impl<T> RangeInclusiveStartWrapper<T> {
    pub fn new(range: RangeInclusive<T>) -> RangeInclusiveStartWrapper<T> {
        RangeInclusiveStartWrapper {
            end_wrapper: RangeInclusiveEndWrapper::new(range),
        }
    }
}

impl<T> PartialEq for RangeInclusiveStartWrapper<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RangeInclusiveStartWrapper<T>) -> bool {
        self.end_wrapper.range.start() == other.end_wrapper.range.start()
    }
}

impl<T> Ord for RangeInclusiveStartWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeInclusiveStartWrapper<T>) -> Ordering {
        self.end_wrapper
            .range
            .start()
            .cmp(other.end_wrapper.range.start())
    }
}

impl<T> PartialOrd for RangeInclusiveStartWrapper<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeInclusiveStartWrapper<T>) -> Option<Ordering> {
        self.end_wrapper
            .range
            .start()
            .partial_cmp(other.end_wrapper.range.start())
    }
}

//
// RangeInclusive end wrapper
//

#[derive(Eq, Debug, Clone)]
pub struct RangeInclusiveEndWrapper<T> {
    pub range: RangeInclusive<T>,
}

impl<T> RangeInclusiveEndWrapper<T> {
    pub fn new(range: RangeInclusive<T>) -> RangeInclusiveEndWrapper<T> {
        RangeInclusiveEndWrapper { range }
    }
}

impl<T> PartialEq for RangeInclusiveEndWrapper<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &RangeInclusiveEndWrapper<T>) -> bool {
        self.range.end() == other.range.end()
    }
}

impl<T> Ord for RangeInclusiveEndWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &RangeInclusiveEndWrapper<T>) -> Ordering {
        self.range.end().cmp(other.range.end())
    }
}

impl<T> PartialOrd for RangeInclusiveEndWrapper<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &RangeInclusiveEndWrapper<T>) -> Option<Ordering> {
        self.range.end().partial_cmp(other.range.end())
    }
}
