// Relies on 'range_contains' feature, which is very soon
// to be stabilised.
//
// https://github.com/rust-lang/rust/issues/32311
//
// Until then, we're tracking nightly.
#![feature(range_contains)]

mod map;
mod set;
mod std_ext;
#[cfg(test)]
mod stupid_range_map;

pub use map::RangeMap;
pub use set::RangeSet;
