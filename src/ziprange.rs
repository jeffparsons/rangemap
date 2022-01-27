// Copyright (c) 2021-2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! The rangemap crate fills an important role, but it's missing a
//! useful way to operate on intersections of two `RangeMap`s that share
//! a key type, which is something we need to do with some frequency,
//! and is also rather annoying to solve generically. This crate
//! facilitates processing multiple `RangeMap`s, by defining a facility
//! for iterating over multiple `RangeMap`s at once, observing where
//! they are disjoint, and where they intersect.

use std::ops::Range;

// Private convenience type, allowing us to work with the return value
// from rangemap::RangeMap<RangeKey, RangeVal>::iter. This is
// intermediate-level Rust:
// * The types of data _returned_ from the Iterator are outlived by the
//   RangeMap that returned them ('RangeMap: 'a).
// * The Iterator object itself is considered to live longer than items
//   returned from it. (This is more strict than necessary, but is
//   adequate for our purposes.) ('Iterator: 'a)
// I'm not sure if my interpretations are correct, the borrow-checker
// is difficult, here.
type RangeZipIterData<'a, RangeKey, RangeVal> =
    Box<dyn Iterator<Item = (&'a Range<RangeKey>, &'a RangeVal)> + 'a>;

// One half of the zipping iterator.
struct RangeZipIterHalf<'a, RangeKey, RangeVal> {
    iter: RangeZipIterData<'a, RangeKey, RangeVal>,
    need_next: bool,
    // The Key needs to be owned, since we update it as we work our way
    // through the intersection.
    cur: Option<(Range<RangeKey>, &'a RangeVal)>,
}
impl<'a, RangeKey, RangeVal> RangeZipIterHalf<'a, RangeKey, RangeVal>
where
    RangeKey: Clone,
{
    fn handle_need_next(&mut self) {
        if self.need_next {
            self.need_next = false;
            self.cur = self.iter.next().map(|(k, v)| (k.clone(), v));
        }
    }
}

/// Iterator over two RangeMap's at once, in range order.
pub struct RangeZipIter<'lhs, 'rhs, RangeKey, LhsRangeVal, RhsRangeVal> {
    lhs: RangeZipIterHalf<'lhs, RangeKey, LhsRangeVal>,
    rhs: RangeZipIterHalf<'rhs, RangeKey, RhsRangeVal>,
}

impl<'lhs, 'rhs, RangeKey, LhsRangeVal, RhsRangeVal>
    RangeZipIter<'lhs, 'rhs, RangeKey, LhsRangeVal, RhsRangeVal>
{
    /// Create a new RangeZipIter out of two iterators that share a
    /// common Key type.
    pub fn new(
        lhs: RangeZipIterData<'lhs, RangeKey, LhsRangeVal>,
        rhs: RangeZipIterData<'rhs, RangeKey, RhsRangeVal>,
    ) -> Self {
        Self {
            lhs: RangeZipIterHalf {
                iter: lhs,
                need_next: true,
                cur: None,
            },
            rhs: RangeZipIterHalf {
                iter: rhs,
                need_next: true,
                cur: None,
            },
        }
    }
}

impl<'lhs, 'rhs, RangeKey, LhsRangeVal, RhsRangeVal> Iterator
    for RangeZipIter<'lhs, 'rhs, RangeKey, LhsRangeVal, RhsRangeVal>
where
    RangeKey: Clone + PartialOrd + Eq,
{
    type Item = (
        Range<RangeKey>,
        (Option<&'lhs LhsRangeVal>, Option<&'rhs RhsRangeVal>),
    );

    fn next(&mut self) -> Option<Self::Item> {
        self.lhs.handle_need_next();
        self.rhs.handle_need_next();
        match (&mut self.lhs.cur, &mut self.rhs.cur) {
            // ran out of both LHS and RHS iterators
            (None, None) => None,
            // ran out of RHS iterator, just report LHS
            (Some((lhs_key, lhs_val)), None) => {
                self.lhs.need_next = true;
                Some((lhs_key.clone(), (Some(lhs_val), None)))
            }
            // ran out of LHS iterator, just report RHS
            (None, Some((rhs_key, rhs_val))) => {
                self.rhs.need_next = true;
                Some((rhs_key.clone(), (None, Some(rhs_val))))
            }
            // both iterators have data
            (Some(ref mut lhs), Some(ref mut rhs)) => {
                if lhs.0.start != rhs.0.start {
                    // they don't start at the same position, so we
                    // report the earlier one, stopping where it ends,
                    // or where it reaches the intersection.
                    let (was_lhs, lesser, greater) =
                        if lhs.0.start < rhs.0.start {
                            (true, &mut lhs.0, &mut rhs.0)
                        } else {
                            (false, &mut rhs.0, &mut lhs.0)
                        };

                    let rval_start = lesser.start.clone();
                    if lesser.end < greater.start {
                        lesser.start = lesser.end.clone();
                    } else {
                        lesser.start = greater.start.clone();
                    };
                    let rval_end = lesser.start.clone();

                    if lesser.start == lesser.end {
                        if was_lhs {
                            self.lhs.need_next = true;
                        } else {
                            self.rhs.need_next = true;
                        };
                    };

                    Some((
                        rval_start..rval_end,
                        if was_lhs {
                            (Some(lhs.1), None)
                        } else {
                            (None, Some(rhs.1))
                        },
                    ))
                } else {
                    // they _did_ start at the same place, so we'll
                    // report both values here, up to wherever the
                    // two entries diverge.
                    let rval_start = lhs.0.start.clone();
                    let rval_end = if lhs.0.end < rhs.0.end {
                        lhs.0.end.clone()
                    } else {
                        rhs.0.end.clone()
                    };

                    if lhs.0.end == rval_end {
                        self.lhs.need_next = true;
                    } else {
                        lhs.0.start = rval_end.clone();
                    };
                    if rhs.0.end == rval_end {
                        self.rhs.need_next = true;
                    } else {
                        rhs.0.start = rval_end.clone();
                    };

                    Some((rval_start..rval_end, (Some(lhs.1), Some(rhs.1))))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rangemap::RangeMap;

    fn collect<'l, 'r, K, L, R>(
        zip_iter: RangeZipIter<'l, 'r, K, L, R>,
    ) -> std::vec::Vec<(Range<K>, (Option<L>, Option<R>))>
    where
        K: Clone + PartialOrd + Eq,
        L: Clone,
        R: Clone,
    {
        zip_iter
            .map(|(k, (l, r))| {
                (k, (l.map(|l| l.clone()), r.map(|r| r.clone())))
            })
            .collect()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Left(u32);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Right(u32);

    #[test]
    fn tests() {
        let mut lhs: RangeMap<usize, Left> = Default::default();
        let mut rhs: RangeMap<usize, Right> = Default::default();

        // left only
        lhs.insert(0..1, Left(0));
        // right only
        rhs.insert(10..11, Right(1));
        // left, then left+right, then left
        lhs.insert(20..23, Left(2));
        rhs.insert(21..22, Right(3));
        // right, then left+right, then right
        rhs.insert(30..33, Right(4));
        lhs.insert(31..32, Left(5));
        // same
        lhs.insert(40..41, Left(6));
        rhs.insert(40..41, Right(7));
        // same start, left longer
        lhs.insert(50..52, Left(8));
        rhs.insert(50..51, Right(9));
        // left starts sooner, end same
        lhs.insert(60..62, Left(10));
        rhs.insert(61..62, Right(11));
        // left, then left+right, then right
        lhs.insert(70..72, Left(12));
        rhs.insert(71..73, Right(13));

        let zip_iter =
            RangeZipIter::new(Box::new(lhs.iter()), Box::new(rhs.iter()));
        let collected: Vec<_> = collect(zip_iter);
        assert_eq!(
            collected,
            vec![
                // left only
                (0..1, (Some(Left(0)), None)),
                // right only
                (10..11, (None, Some(Right(1)))),
                // left, then left+right, then left
                (20..21, (Some(Left(2)), None)),
                (21..22, (Some(Left(2)), Some(Right(3)))),
                (22..23, (Some(Left(2)), None)),
                // right, then left+right, then right
                (30..31, (None, Some(Right(4)))),
                (31..32, (Some(Left(5)), Some(Right(4)))),
                (32..33, (None, Some(Right(4)))),
                // same
                (40..41, (Some(Left(6)), Some(Right(7)))),
                // same start, left longer
                (50..51, (Some(Left(8)), Some(Right(9)))),
                (51..52, (Some(Left(8)), None)),
                // left starts sooner, end same
                (60..61, (Some(Left(10)), None)),
                (61..62, (Some(Left(10)), Some(Right(11)))),
                // left, then left+right, then right
                (70..71, (Some(Left(12)), None)),
                (71..72, (Some(Left(12)), Some(Right(13)))),
                (72..73, (None, Some(Right(13)))),
            ]
        );
    }
}
