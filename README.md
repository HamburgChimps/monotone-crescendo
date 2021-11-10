# Montotone Crescendo
 
This is an implementation of [LeetCode problem #926](https://leetcode.com/problems/flip-string-to-monotone-increasing/) in Rust.

It is also a playground/demonstration of compiling Rust to WebAssembly and passing a string from Javascript to the compiled Rust program, and from the compiled Rust program back to Javascript.

[See a working demo](https://hamburgchimps.github.io/monotone-crescendo/).

Huge credit to Dr. Richard Apodaca and his blog, [depth-first.com](https://depth-first.com), without which I'm not sure I would have been able to make sense of how to read and write from WebAssembly's linear memory without having to dive straight into something like [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen).

These two blog posts from Dr. Apodaca were most helpful:

1. [Compiling Rust to WebAssembly: A Simple Example](https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/)
2. [Rust and WebAssembly from Scratch: Hello World with Strings](https://depth-first.com/articles/2020/07/07/rust-and-webassembly-from-scratch-hello-world-with-strings/)