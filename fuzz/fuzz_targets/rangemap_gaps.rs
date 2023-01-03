#![feature(array_windows)]
#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::{Arbitrary, Unstructured};
use rangemap::RangeMap;
use std::ops::Range;

#[derive(Clone, Debug, Arbitrary)]
enum Op {
    Insert(Range<u8>, u8),
    Remove(Range<u8>),
}

impl Op {
    fn apply(self, map: &mut RangeMap<u8, u8>) {
        match self {
            Op::Insert(r, v) if r.start < r.end => map.insert(r, v),
            Op::Remove(r) if r.start < r.end => map.remove(r),
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    ops: Vec<Op>,
    outer_range: Range<u8>,
}

impl Arbitrary for Input {
    fn arbitrary(u: &mut Unstructured) -> arbitrary::Result<Self> {
        Ok(Self {
            ops: u.arbitrary()?,
            // Larger margins than these are too
            // far away from boundary conditions to be interesting.
            // ("Oh, the fools! If only they'd built it with 6,001 hulls." -- Philip J. Fry)
            //
            // NOTE: Not using `int_in_range` because of <https://github.com/rust-fuzz/arbitrary/issues/106>.
            outer_range: *u.choose(&[0, 1, 2, 3, 100, 101, 102, 103])?
                ..*u.choose(&[100, 101, 102, 103, 252, 253, 254, 255])?,
        })
    }
}

fuzz_target!(|input: Input| {
    let Input { ops, outer_range } = input;

    let mut map = RangeMap::new();

    for op in ops {
        op.apply(&mut map);
    }

    // Check that the combination of gaps and keys fills the entire outer range.
    let gaps: Vec<Range<u8>> = map.gaps(&outer_range).collect();
    let mut keys: Vec<Range<u8>> = map
        .overlapping(&outer_range)
        .map(|(Range { start, end }, _key)| {
            // Truncate anything straddling either edge.
            u8::max(*start, outer_range.start)..u8::min(*end, outer_range.end)
        })
        .filter(|range| {
            // Reject anything that is now empty after being truncated.
            !range.is_empty()
        })
        .collect();

    keys.extend(gaps.into_iter());
    keys.sort_by_key(|key| key.start);

    if outer_range.is_empty() {
        // There should be no gaps or keys returned if the outer range is empty,
        // because empty ranges cover no values.
        assert!(keys.is_empty());
        return;
    }

    // Gaps and keys combined should span whole outer range.
    assert_eq!(keys.first().unwrap().start, outer_range.start);
    assert_eq!(keys.last().unwrap().end, outer_range.end);

    // Each gap/key should start where the previous one ended.
    for [a, b] in keys.array_windows::<2>() {
        assert_eq!(a.end, b.start);
    }
});
