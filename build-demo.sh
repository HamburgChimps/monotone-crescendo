#!/usr/bin/env bash -x

# build demo directory
mkdir -p demo
cargo build --target wasm32-unknown-unknown -- release
cargo doc
cp target/wasm32-unknown-unknown/release/monotone_crescendo.wasm demo/
cp -r target/doc demo
cp demo.html demo/index.html
