#!/usr/bin/env bash -x

# build demo directory
rm -rf demo
mkdir demo
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/monotone_crescendo.wasm
cargo doc
cp target/wasm32-unknown-unknown/release/monotone_crescendo.wasm demo/
cp -r target/doc demo
cp demo.html demo/index.html
