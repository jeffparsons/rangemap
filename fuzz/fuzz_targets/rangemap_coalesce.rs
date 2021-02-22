#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
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

fuzz_target!(|ops: Vec<Op>| {
    let mut map = RangeMap::new();

    for op in ops {
        op.apply(&mut map);
    }

    let mut peek = map.iter().peekable();
    while let Some((range, val)) = peek.next() {
        if let Some((nextrange, nextval)) = peek.peek() {
            if range.end == nextrange.start && val == *nextval {
                panic!()
            }
        }
    }
});
