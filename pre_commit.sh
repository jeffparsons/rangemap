#!/bin/bash

set -ex

cargo test
# For doc tests in "README.md".
cargo +nightly test --features nightly
cargo run --example roster
cargo fmt
cargo clippy --bins --examples --tests --benches -- -D warnings
