# rangemap

[![Crate](https://img.shields.io/crates/v/rangemap.svg)](https://crates.io/crates/rangemap)
[![Docs](https://docs.rs/rangemap/badge.svg)](https://docs.rs/rangemap)
[![Build status](https://travis-ci.org/jeffparsons/rangemap.svg?branch=master)](https://travis-ci.org/jeffparsons/rangemap)
[![Build status](https://ci.appveyor.com/api/projects/status/github/jeffparsons/rangemap?svg=true)](https://ci.appveyor.com/project/jeffparsons/rangemap)
[![Rust](https://img.shields.io/badge/rust-1.32%2B-blue.svg?maxAge=3600)](https://github.com/jeffparsons/rangemap) <!-- Don't forget to update the Travis config when bumping minimum Rust version. -->


[RangeMap](https://docs.rs/rangemap/latest/rangemap/struct.RangeMap.html) is a map data structure whose keys are stored as ranges. Contiguous and overlapping ranges that map to the same value are coalesced into a single range.

A corresponding [RangeSet](https://docs.rs/rangemap/latest/rangemap/struct.RangeSet.html) structure is also provided.
