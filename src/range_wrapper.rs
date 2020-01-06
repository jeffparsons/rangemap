// Wrapper to allow storing (and sorting/searching)
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

use std::cmp::Ordering;
use std::ops::Range;

//
// Range start wrapper
//

#[derive(Eq, Debug, Clone)]
pub struct StartWrapper<T> {
    pub range: Range<T>,
}

impl<T> StartWrapper<T> {
    pub fn new(range: Range<T>) -> StartWrapper<T> {
        StartWrapper { range }
    }
}

impl<T> PartialEq for StartWrapper<T>
where
    T: Eq,
{
    fn eq(&self, other: &StartWrapper<T>) -> bool {
        self.range.start == other.range.start
    }
}

impl<T> Ord for StartWrapper<T>
where
    T: Ord,
{
    fn cmp(&self, other: &StartWrapper<T>) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl<T> PartialOrd for StartWrapper<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &StartWrapper<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
