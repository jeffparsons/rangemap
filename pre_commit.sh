#!/bin/bash

set -ex

cargo test
cargo fmt
cargo clippy --bins --examples --tests --benches -- -D warnings
