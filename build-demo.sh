#!/usr/bin/env bash -x

# build demo directory
mkdir demo
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/monotone_crescendo.wasm
cago doc
cp target/wasm32-unknown-unknown/release/monotone_crescendo.wasm demo/
cp target/doc demo
cp demo.html demo/index.html
