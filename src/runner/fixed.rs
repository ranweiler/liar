use ::Sample;
use black_box::black_box;
use runner::Runner;
use timer::{CPUTimer, Timer};


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

    fn run_round<Target, Ret>(&mut self, round_size: usize, target: &mut Target) -> f64
        where Target: FnMut() -> Ret {

        let mut start = CPUTimer::new();
        let mut end = CPUTimer::new();

        start.mark();
        for _ in 0..round_size {
            black_box(target());
        }
        end.mark();

        end.since(&start) / (round_size as f64)
    }
}

impl Runner<f64> for FixedRunner {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<f64>
        where Target: FnMut() -> Ret {

        let mut data = Vec::with_capacity(self.sample_size);

        let round_size = self.round_size;  // For borrowck.
        for _ in 0..self.sample_size {
            data.push(self.run_round(round_size, target))
        }

        Sample { name, data }
    }
}
