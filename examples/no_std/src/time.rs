use libc;


pub fn time() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    }
    (ts.tv_sec * 1_000_000_000 + ts.tv_nsec) as u64
}

pub fn diff(start: &u64, end: &u64) -> u64 {
    end - start
}
