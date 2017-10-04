use std::time::{Instant, Duration};

use black_box::black_box;


fn ns_from_dur(dur: &Duration) -> u64 {
    let ns_per_sec = 1_000_000_000_u64;
    (dur.as_secs() as u64) * ns_per_sec + (dur.subsec_nanos() as u64)
}

pub const DEFAULT_ROUNDS: usize = 10_000;
pub const DEFAULT_SAMPLE_SIZE: usize = 100;

pub struct Samples {
    pub name: &'static str,
    pub data: Vec<u64>,
}

pub struct FixedRunner {
    rounds: usize,
    sample_size: usize,
}

impl FixedRunner {
    pub fn new(rounds: usize, sample_size: usize) -> Self {
        FixedRunner {
            rounds,
            sample_size,
        }
    }

    pub fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Samples
        where Target: FnMut() -> Ret {

        let mut data = Vec::with_capacity(self.sample_size);

        let rounds = self.rounds;  // For borrowck.
        for _ in 0..self.sample_size {
            data.push(self.run_rounds(rounds, target))
        }

        Samples { name, data }
    }

    fn run_rounds<Target, Ret>(&mut self, rounds: usize, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let now = Instant::now();
        for _ in 0..rounds {
            black_box(target());
        }
        let dur = now.elapsed();

        ns_from_dur(&dur) / (rounds as u64)
    }
}
