### v1.4.0 (2023-09-19)

- **Changes**:
    - Bump minimum supported Rust version to 1.61.0 (released 2022-05-19). This is for compatibility with new versions of some of rangemap's test dependencies.


### v1.3.1 (2023-09-19)

- **Fixes**:
    - Fix `PartialEq`, `PartialOrd`, and `Ord` implementations for `RangeMap` (and `RangeSet`). These previously only compared the starts of ranges instead of the entire range. Thanks to https://github.com/hecrj for finding and fixing this!
- **Changes**:
    - Minimum supported Rust version for running this crate's tests has increased to 1.61.0 due to a corresponding MSRV bump in a test dependency. This is causing CI builds to fail against rustc 1.56.1, but should not affect consumers of the crate. The next minor release (1.4.0) will bump the minimum supported Rust version of rangemap itself.


### v1.3.0 (2023-01-03)

- **Features**:
    - Add `overlapping` method to all collection types, which returns an iterator over all stored entries that completely or partially overlap a given range.
    - Add `overlaps` convenience method to all collection types, which returns whether any stored range completely or partially overlaps a given range.
    - Credit to [Rua](https://github.com/Rua) for the original implementation of these new methods. (Unfortunately I couldn't use their code directly because I made other incompatible changes.) Thanks also to [rumpuslabs](https://github.com/rumpuslabs) for their engagement.


### v1.2.0 (2022-12-27)

- **Features**:
    - Add `clear`, `len`, and `is_empty` methods for all map and set types.
- **Fixes**:
    - Make `const_fn` feature work again. (Previous release accidentally broke the const versions of `RangeMap::new` and `RangeSet::new`.)


### v1.1.0 (2022-11-12)

- **Features**:
    - Implement `PartialEq`, `Eq`, `PartialOrd`, and `Ord` for all map and set types.
    - Make `new` functions for all map and set types `const` when `const_fn` feature is enabled.
- **Changes**:
    - Bump minimum supported Rust version to 1.56.1. (Released a year ago.) This is for compatibility with new versions of some of rangemap's development dependencies.


### v1.0.3 (2022-06-11)

- **Fixes**:
    - Fix `Gaps` iterator for `RangeMap` yielding an empty gap for an empty outer range. Simplified gaps logic and expanded fuzz testing to better cover this and similar cases.


### v1.0.2 (2022-05-17)

- **Fixes**:
    - Fix empty gaps returned by `Gaps` iterator for `RangeInclusiveMap`. Added fuzz tests for `Gaps` iterators.


### v1.0.1 (2022-01-29)

- **Fixes**:
    - Fix empty gaps returned by `Gaps` iterator for `RangeMap`, and incorrect gaps returned by `Gaps` iterator for `RangeInclusiveMap`.


### v1.0.0 (2022-01-28)

It's time. (No functional change.)


### v0.1.14 (2021-11-16)

- **Features**:
    - Expose nameable types for iterators: `Iterator`, `IntoIterator`, `Gaps` (for each collection type).
- **Changes**:
    - Document overflow behaviour required by implementors of `StepLite` and `StepFns`.


### v0.1.13 (2021-08-25)

- **Features**:
    - Add serde support.


### v0.1.12 (2021-08-23)

- **Features**:
    - Implement more traits for all map and set types: `IntoIter`, `FromIter`, and `Extend`.
- **Changes**:
    - Bump minimum supported Rust version to 1.46.


### v0.1.11 (2021-06-30) "EOFY edition"

- **Features**:
    - Support `no_std` environments.
- **Changes**:
    - Update all dev-dependencies to latest versions.


### v0.1.10 (2021-02-23)

- **Fixes**:
    - Fix performance regression introduced in v0.1.9, which made inserts extremely slow for large maps.


### v0.1.9 (2021-02-23)

- **Fixes**:
    - Fix coalescing of contiguous ranges. In some cases `RangeMap` and `RangeInclusiveMap` would leave two separate contiguous ranges with the same value instead of combining them into one.


### v0.1.8 (2020-11-22)

- **Features**:
    - Implement `Debug` for all map and set types.


### v0.1.7 (2020-09-07)

- **Features**:
    - Add `gaps` method to all map and set types for iterating over sub-ranges of a given outer range that are not covered by any stored range.


### v0.1.6 (2020-07-15)

- **Features**:
    - Add `RangeInclusiveMap` and `RangeInclusiveSet` types for storing closed ranges.
