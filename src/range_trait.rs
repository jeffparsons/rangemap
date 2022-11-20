use core::ops::Range;

pub trait RangeTrait {
    type A: Ord;
    fn start(&self) -> &Self::A;
    fn end(&self) -> &Self::A;
    fn set_start(&mut self, new_start: Self::A);
    fn set_end(&mut self, new_end: Self::A);
    fn new(start: Self::A, end: Self::A) -> Self;
    fn contains(&self, item: &Self::A) -> bool {
        item >= self.start() && item < self.end()
    }
    fn overlaps(&self, other: &Self) -> bool {
        use core::cmp::{max, min};
        // Strictly less than, because ends are excluded.
        max(&self.start(), &other.start()) < min(&self.end(), &other.end())
    }
    fn touches(&self, other: &Self) -> bool {
        use core::cmp::{max, min};
        // Less-than-or-equal-to because if one end is excluded, the other is included.
        // I.e. the two could be joined into a single range, because they're overlapping
        // or immediately adjacent.
        max(&self.start(), &other.start()) <= min(&self.end(), &other.end())
    }
}

// Implement for all ranges
impl<T> RangeTrait for Range<T>
where
    T: Ord,
{
    type A = T;
    fn new(start: Self::A, end: Self::A) -> Self {
        start..end
    }
    fn start(&self) -> &Self::A {
        &self.start
    }
    fn end(&self) -> &Self::A {
        &self.end
    }
    fn set_start(&mut self, new_start: Self::A) {
        self.start = new_start;
    }
    fn set_end(&mut self, new_end: Self::A) {
        self.end = new_end;
    }
}
