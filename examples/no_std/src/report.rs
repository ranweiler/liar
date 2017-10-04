use core::fmt::{self, Write};

use libc;

use liar::no_std::runner::Sample;


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

pub fn report(s: &Sample) {
    let mut total = 0f64;
    for i in 0..s.data.len() {
        total += s.data[i] as f64;
    }
    let n = s.data.len() as f64;
    let avg = total / n;

    writeln!(Stdout, "[{}]\t{}", s.name, avg).ok();
}
