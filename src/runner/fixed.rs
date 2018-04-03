//! Measurement of average call time using fixed-size execution rounds.

use std::marker::PhantomData;

use ::Sample;
use black_box::black_box;
use runner::Runner;
use timer::Timer;


/// Default number of executions per round.
pub const DEFAULT_ROUND_SIZE: usize = 10_000;

/// Default number of rounds to run and record.
pub const DEFAULT_SAMPLE_SIZE: usize = 100;

/// Runs multiple rounds of executions of a target, recording the average target
/// execution time in each round.
pub struct FixedRunner<T: Timer> {
    round_size: usize,
    sample_size: usize,
    _timer: PhantomData<T>,
}

impl<T: Timer> FixedRunner<T> {
    /// Construct a `FixedRunner` that will record a sample of its target with
    /// `sample_size` timing measurements. Each measurement will be the average
    /// time per call after executing the target `round_size` times.
    pub fn new(round_size: usize, sample_size: usize) -> Self {
        FixedRunner {
            round_size,
            sample_size,
            _timer: PhantomData,
        }
    }

    fn run_round<Target, Ret>(&mut self, round_size: usize, target: &mut Target) -> f64
        where Target: FnMut() -> Ret {

        let mut start = T::new_timer();
        let mut end = T::new_timer();

        start.mark();
        for _ in 0..round_size {
            black_box(target());
        }
        end.mark();

        end.since(&start) / (round_size as f64)
    }
}

impl<T: Timer> Runner<f64> for FixedRunner<T> {
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
