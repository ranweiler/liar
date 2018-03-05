// Copyright 2012-2017 The Rust Project Developers. See the COPYRIGHT
// file at https://rust-lang.org/COPYRIGHT.
//
// Licensed under the the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.
//
// The `asm!`-enabled implementation of `black_box` comes from the
// upstream `libtest` crate by The Rust Project Developers. The
// non-`asm!` implementation is by bluss, and comes from the `bencher`
// crate, which is port of `libtest` to stable Rust. Both are
// dual-licensed, and in each case we incorporate the relevant code
// using the MIT License option.

/// A function that is opaque to the optimizer, to allow benchmarks to
/// pretend to use outputs to assist in avoiding dead-code elimination.
///
/// This function is a no-op, and the `asm!` implementation does not
/// even read from `x`.
#[cfg(asm)]
pub fn black_box<T>(x: T) -> T {
    // We need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&x)) }
    x
}

#[cfg(not(asm))]
#[inline(never)]
pub fn black_box<T>(x: T) -> T {
    use std::mem;
    use std::ptr;

    unsafe {
        let ret = ptr::read_volatile(&x);
        mem::forget(x);
        ret
    }
}
