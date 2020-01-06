/*!
[RangeMap](struct.RangeMap.html) is a map data structure whose keys are
stored as ranges. Contiguous and overlapping ranges
that map to the same value are coalesced into a single range.

A corresponding [RangeSet](struct.RangeSet.html) structure is also provided.


# Example: use with Chrono

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

*/

mod map;
mod range_wrapper;
mod set;
mod std_ext;
#[cfg(test)]
mod stupid_range_map;

pub use map::RangeMap;
pub use set::RangeSet;
