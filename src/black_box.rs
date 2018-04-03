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
//! A function that is opaque to the optimizer, to allow benchmarks to
//! pretend to use outputs to assist in avoiding dead-code elimination.


/// Mark a value as used to prevent it from being optimized away.
#[cfg(asm)]
pub fn black_box<T>(x: T) -> T {
    // "Use" the argument in some way that the optimizer can't inspect.
    unsafe { asm!("" : : "r"(&x)) }
    x
}

#[cfg(not(asm))]
#[inline(never)]
/// Mark a value as used to prevent it from being optimized away.
pub fn black_box<T>(x: T) -> T {
    use std::mem;
    use std::ptr;

    unsafe {
        // Use a more expensive method when inline asm is unavailable.
        let ret = ptr::read_volatile(&x);
        mem::forget(x);
        ret
    }
}
