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
