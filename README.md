# rangemap

[![Crate](https://img.shields.io/crates/v/rangemap.svg)](https://crates.io/crates/rangemap)
[![Docs](https://docs.rs/rangemap/badge.svg)](https://docs.rs/rangemap)
[![Build status](https://travis-ci.org/jeffparsons/rangemap.svg?branch=master)](https://travis-ci.org/jeffparsons/rangemap)
[![Build status](https://ci.appveyor.com/api/projects/status/github/jeffparsons/rangemap?svg=true)](https://ci.appveyor.com/project/jeffparsons/rangemap)
[![Rust](https://img.shields.io/badge/rust-1.43%2B-blue.svg?maxAge=3600)](https://github.com/jeffparsons/rangemap) <!-- Don't forget to update the Travis config when bumping minimum Rust version. -->

[`RangeMap`] and [`RangeInclusiveMap`] are map data structures whose keys
are stored as ranges. Contiguous and overlapping ranges that map to the same
value are coalesced into a single range.

Corresponding [`RangeSet`] and [`RangeInclusiveSet`] structures are also provided.

[`RangeMap`]: https://docs.rs/rangemap/latest/rangemap/struct.RangeMap.html
[`RangeInclusiveMap`]: https://docs.rs/rangemap/latest/rangemap/struct.RangeInclusiveMap.html
[`RangeSet`]: https://docs.rs/rangemap/latest/rangemap/struct.RangeSet.html
[`RangeInclusiveSet`]: https://docs.rs/rangemap/latest/rangemap/struct.RangeInclusiveSet.html


## Example: use with Chrono

```rust
use chrono::offset::TimeZone;
use chrono::{Duration, Utc};
use rangemap::RangeMap;

fn main() {
    let people = ["Alice", "Bob", "Carol"];
    let mut roster = RangeMap::new();

    // Set up initial roster.
    let start_of_roster = Utc.ymd(2019, 1, 7);
    let mut week_start = start_of_roster;
    for _ in 0..3 {
        for person in &people {
            let next_week = week_start + Duration::weeks(1);
            roster.insert(week_start..next_week, person);
            week_start = next_week;
        }
    }

    // Bob is covering Alice's second shift (the fourth shift overall).
    let fourth_shift_start = start_of_roster + Duration::weeks(3);
    let fourth_shift_end = fourth_shift_start + Duration::weeks(1);
    roster.insert(fourth_shift_start..fourth_shift_end, &"Bob");

    for (range, person) in roster.iter() {
        println!("{} ({}): {}", range.start, range.end - range.start, person);
    }

    // Output:
    // 2019-01-07UTC (P7D): Alice
    // 2019-01-14UTC (P7D): Bob
    // 2019-01-21UTC (P7D): Carol
    // 2019-01-28UTC (P14D): Bob
    // 2019-02-11UTC (P7D): Carol
    // 2019-02-18UTC (P7D): Alice
    // 2019-02-25UTC (P7D): Bob
    // 2019-03-04UTC (P7D): Carol
}
```
