# Liar

Liar is a Rust library to support writing benchmark suites as
stand-alone binaries.

The project goals are:

- Support reuse of benchmarks across both standard OS and
  embedded/`#![no_std]` targets.
- Enable benchmark execution on a target with post-hoc analysis of raw
  data on a host.
- Let users control how benchmarks are executed, what data are
  reported, and the means of reporting.
- Compile with stable Rust.

## License

Liar is released under the [ISC License](LICENSE). The implementations
of [`black_box`](src/black_box.rs) are derived from the
[`bencher`](https://crates.io/crates/bencher) and Rust
[`libtest`](https://github.com/rust-lang/rust/tree/master/src/libtest)
crates, which we use under the [MIT license](LICENSE-MIT).
