#!/usr/bin/env bash -x

# build demo directory
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/monotone_crescendo.wasm
cp target/wasm32-unknown-unknown/release/monotone_crescendo.wasm demo/
