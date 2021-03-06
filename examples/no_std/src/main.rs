#![no_std]

#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(start)]

extern crate compiler_builtins;
extern crate libc;
extern crate liar;

mod report;
mod suite;
mod time;


#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    suite::main();
    0
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[allow(private_no_mangle_fns)]
#[lang = "panic_fmt"]
#[no_mangle]  // Fixes link error: `undefined reference to `rust_begin_unwind'`
extern fn panic_fmt() -> ! { loop {} }
