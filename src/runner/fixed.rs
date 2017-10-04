use std::time::{Instant, Duration};

use ::Sample;
use black_box::black_box;


fn ns_from_dur(dur: &Duration) -> u64 {
    let ns_per_sec = 1_000_000_000_u64;
    (dur.as_secs() as u64) * ns_per_sec + (dur.subsec_nanos() as u64)
}

pub const DEFAULT_ROUND_SIZE: usize = 10_000;
pub const DEFAULT_SAMPLE_SIZE: usize = 100;

pub struct FixedRunner {
    round_size: usize,
    sample_size: usize,
}

impl FixedRunner {
    pub fn new(round_size: usize, sample_size: usize) -> Self {
        FixedRunner {
            round_size,
            sample_size,
        }
    }

    pub fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<u64>
        where Target: FnMut() -> Ret {

        let mut data = Vec::with_capacity(self.sample_size);

        let round_size = self.round_size;  // For borrowck.
        for _ in 0..self.sample_size {
            data.push(self.run_round(round_size, target))
        }

        Sample { name, data }
    }

    fn run_round<Target, Ret>(&mut self, round_size: usize, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let now = Instant::now();
        for _ in 0..round_size {
            black_box(target());
        }
        let dur = now.elapsed();

        ns_from_dur(&dur) / (round_size as u64)
    }
}
