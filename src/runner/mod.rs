pub mod fixed;

use std::time::Duration;

use ::Sample;


pub trait Runner<S> {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<S>
        where Target: FnMut() -> Ret;
}

fn to_ns(d: &Duration) -> u64 {
    let ns_per_sec: u64 = 1_000_000_000;
    (d.as_secs() * ns_per_sec) + (d.subsec_nanos() as u64)
}
