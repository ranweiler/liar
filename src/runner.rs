use std::time::{Instant, Duration};

use black_box::black_box;


pub const SAMPLE_SIZE: usize = 100;

pub struct Samples {
    pub name: &'static str,
    pub data: [u64; SAMPLE_SIZE],
}

pub struct Runner {}

fn ns_from_dur(dur: &Duration) -> u64 {
    let ns_per_sec = 1_000_000_000_u64;
    (dur.as_secs() as u64) * ns_per_sec + (dur.subsec_nanos() as u64)
}

impl Runner {
    pub fn new() -> Self {
        Runner {}
    }

    pub fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Samples
        where Target: FnMut() -> Ret {

        let mut data = [0_u64; SAMPLE_SIZE];

        for ix in 0..SAMPLE_SIZE {
            data[ix] = self.run_loop(target);
        }

        Samples { name, data }
    }

    fn run_loop<Target, Ret>(&mut self, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let count = 10_000_u64;

        let now = Instant::now();
        for _ in 0..count {
            black_box(target());
        }
        let dur = now.elapsed();

        ns_from_dur(&dur) / count
    }
}
