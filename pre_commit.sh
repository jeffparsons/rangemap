#!/bin/bash

set -ex

cargo test
cargo run --example roster
cargo fmt
cargo clippy --bins --examples --tests --benches -- -D warnings
