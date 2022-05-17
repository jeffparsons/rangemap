# rangemap fuzz tests

These fuzz tests use the [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) tool.

Run one of these:

```
cargo +nightly fuzz run rangemap_coalesce
cargo +nightly fuzz run rangemap_inclusive_coalesce
cargo +nightly fuzz run rangemap_gaps
cargo +nightly fuzz run rangemap_inclusive_gaps
```
