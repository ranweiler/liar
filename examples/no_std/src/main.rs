#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(start)]
#![no_std]
extern crate compiler_builtins;
extern crate libc;
extern crate liar;

use liar::no_std::bencher::Bencher;
use liar::no_std::runner::Samples;

use core::fmt::{self, Write};


// From: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ret = unsafe {
            libc::write(libc::STDOUT_FILENO,
                        s.as_ptr() as *const _, s.len())
        };
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

mod acker {
    pub fn mann(m: usize, n: usize) -> usize {
        match m {
            0 => n + 1,
            _ => match n {
                0 => mann(m - 1, 1),
                n => mann(m - 1, mann(m, n - 1)),
            }
        }
    }

}

fn nop(b: &mut Bencher<u64>) {
    b.run(|| {});
}

fn zeroize(b: &mut Bencher<u64>) {
    const BUF_LEN: usize = 1024;
    let mut buf = [0u8; BUF_LEN];

    b.run(|| {
        for i in 0..BUF_LEN {
            buf[i] = 0;
        }
        buf
    });
}

fn ack(b: &mut Bencher<u64>) {
    b.run(|| {
        acker::mann(3, 2)
    });
}

fn time() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    }
    (ts.tv_sec * 1_000_000_000 + ts.tv_nsec) as u64
}

fn diff(start: &u64, end: &u64) -> u64 {
    end - start
}

fn report(s: &Samples) {
    let mut total = 0f64;
    for i in 0..s.data.len() {
        total += s.data[i] as f64;
    }
    let n = s.data.len() as f64;
    let avg = total / n;

    writeln!(Stdout, "[{}]\t{}", s.name, avg).ok();
}

const SAMPLE_SIZE: usize = 100;

// Entry point for this program
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let mut data = [0u64; SAMPLE_SIZE];
    let mut b = Bencher::new(&mut data, time, diff);

    let mut out = [0u64; SAMPLE_SIZE];
    report(&b.bench("nop", &mut nop, &mut out));
    report(&b.bench("zeroize", &mut zeroize, &mut out));
    report(&b.bench("ack", &mut ack, &mut out));

    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]  // Fixes link error: `undefined reference to `rust_begin_unwind'`
extern fn panic_fmt() -> ! { loop {} }
