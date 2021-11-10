#!/usr/bin/env bash -xe

# build for gh-pages
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/monotone_crescendo.wasm
cp target/wasm32-unknown-unknown/release/monotone_crescendo.wasm demo/
git checkout gh-pages
git checkout main -- demo
cp demo/* .
git reset HEAD demo
git commit -am 'update demo'
git push && git checkout main || git checkout main
