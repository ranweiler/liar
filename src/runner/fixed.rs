use std::time::Instant;

use Sample;
use runner::{to_ns, Runnable, Runner};
use black_box::black_box;

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

    fn run_round<Target, Ret>(&mut self, round_size: usize, target: &mut Target) -> u64
    where
        Target: Runnable<Ret>,
    {
        let now = Instant::now();
        for _ in 0..round_size {
            black_box(target.body());
        }
        let dur = now.elapsed();

        to_ns(&dur) / (round_size as u64)
    }
}

impl Runner<u64> for FixedRunner {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<u64>
    where
        Target: Runnable<Ret>,
    {
        let mut data = Vec::with_capacity(self.sample_size);

        let round_size = self.round_size; // For borrowck.
        for _ in 0..self.sample_size {
            data.push(self.run_round(round_size, target))
        }

        Sample { name, data }
    }
}
