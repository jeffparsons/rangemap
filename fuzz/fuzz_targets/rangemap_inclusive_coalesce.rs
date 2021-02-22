#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use rangemap::RangeInclusiveMap;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Arbitrary)]
enum Op {
    Insert(RangeInclusive<u8>, u8),
    Remove(RangeInclusive<u8>),
}

impl Op {
    fn apply(self, map: &mut RangeInclusiveMap<u8, u8>) {
        match self {
            Op::Insert(r, v) => map.insert(r, v),
            Op::Remove(r) => map.remove(r),
        }
    }
}

fuzz_target!(|ops: Vec<Op>| {
    let mut map = RangeInclusiveMap::new();

    for op in ops {
        op.apply(&mut map);
    }

    let mut peek = map.iter().peekable();
    while let Some((range, val)) = peek.next() {
        if let Some((nextrange, nextval)) = peek.peek() {
            if *range.end() == nextrange.start() - 1 && val == *nextval {
                panic!()
            }
        }
    }
});
