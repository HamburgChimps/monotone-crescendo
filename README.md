# Monotone Crescendo

Rust implementations of [LeetCode problem #926](https://leetcode.com/problems/flip-string-to-monotone-increasing/), compiled to WebAssembly and invoked via Javascript

[See a working demo](https://hamburgchimps.github.io/monotone-crescendo/).

## How To Try It Out Locally

1. Clone this repository onto your machine.
2. Install [`wasm-gc`](https://github.com/alexcrichton/wasm-gc) by running `cargo install wasm-gc`. This is used by [` build-demo.sh`](build-demo.sh) to reduce the size of the compiled wasm binary by removing unneccesary/unused cruft. Even
though the `wasm-gc` project itself says you shouldn't use it in most cases, it still seems to get the wasm file the smallest. I tried using the `--gc-sections` flag in the compiler options and `wasm-gc` still got it smaller. I am not making use of [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) or [`wasm-pack`](https://github.com/rustwasm/wasm-pack) in this project, which both attempt to remove cruft when compiling, so I utilized `wasm-gc` to do it manually.
3. Run `./build-demo.sh` in the repository root. This creates the `monotone_crescendo.wasm` binary and an `index.html` file in a directory named `demo` in the repository root.
4. `cd` into `demo/` and run an http server. Using python's `SimpleHTTPServer`, for example:
```bash
cd demo/
python -m SimpleHTTPServer
```
The demo will then be available at `<localhost:8000>`.


## Crate Documentation

Detailed documnetation generated via [rustdoc](https://doc.rust-lang.org/rustdoc/index.html) can be found [alongside the demo](https://hamburgchimps.github.io/monotone-crescendo/doc/monotone_crescendo).

## Acknowledgements

Huge credit to Dr. Richard Apodaca and his blog, [depth-first.com](https://depth-first.com), without which I'm not sure I would have been able to make sense of how to read and write from WebAssembly's linear memory without having to dive straight into something like [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen).

These two blog posts from Dr. Apodaca were most helpful:

1. [Compiling Rust to WebAssembly: A Simple Example](https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/)
2. [Rust and WebAssembly from Scratch: Hello World with Strings](https://depth-first.com/articles/2020/07/07/rust-and-webassembly-from-scratch-hello-world-with-strings/)

Further credit goes to [Radu Matei](https://radu-matei.com) and his [blog post](https://radu-matei.com/blog/practical-guide-to-wasm-memory), which helped me build upon the concepts I learned from Dr. Apodaca's posts.

The [prefix sum solution](https://hamburgchimps.github.io/monotone-crescendo/doc/monotone_crescendo/solution/fn.monotone_crescendo_prefix_sums.html) is the [official solution on LeetCode](https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/), I only translated it into Rust.

The [cumulative solution](https://hamburgchimps.github.io/monotone-crescendo/doc/monotone_crescendo/solution/fn.monototone_crescendo_cumulative.html) was [posted to LeetCode](https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/725259) by [tarunbisht](https://leetcode.com/tarunbisht), and was translated into Rust by me.
