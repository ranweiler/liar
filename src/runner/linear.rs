use std::time::Instant;

use Sample;
use runner::{to_ns, Round, Runnable, Runner};
use black_box::black_box;

pub const DEFAULT_ROUND_SIZE_START: usize = 1_000;
pub const DEFAULT_ROUND_SIZE_STEP: usize = 100;
pub const DEFAULT_SAMPLE_SIZE: usize = 100;

pub struct LinearRunner {
    round_size_start: usize,
    round_size_step: usize,
    sample_size: usize,
}

impl LinearRunner {
    pub fn new(round_size_start: usize, round_size_step: usize, sample_size: usize) -> Self {
        LinearRunner {
            round_size_start,
            round_size_step,
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

        to_ns(&dur)
    }
}

impl Runner<Round> for LinearRunner {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<Round>
    where
        Target: Runnable<Ret>,
    {
        let mut data = Vec::with_capacity(self.sample_size);

        let mut round_size = self.round_size_start;
        for _ in 0..self.sample_size {
            data.push(Round {
                ns: self.run_round(round_size, target),
                size: round_size,
            });

            round_size += self.round_size_step;
        }

        Sample { name, data }
    }
}
