//! Measurement of call time using rounds with an linearly-increasing size.

use std::marker::PhantomData;

use ::Sample;
use black_box::black_box;
use runner::{Round, Runner};
use timer::Timer;


/// Default number of executions in first round.
pub const DEFAULT_ROUND_SIZE_START: usize = 1_000;

/// Default increase in round size between rounds.
pub const DEFAULT_ROUND_SIZE_STEP: usize = 100;

/// Defualt total number of rounds to record.
pub const DEFAULT_SAMPLE_SIZE: usize = 100;

/// Run a target in linearly-increasing round sizes. These can then be analyzed
/// with, for example, linear regression.
pub struct LinearRunner<T: Timer> {
    round_size_start: usize,
    round_size_step: usize,
    sample_size: usize,
    _timer: PhantomData<T>,
}

impl<T: Timer> LinearRunner<T> {
    /// Construct a `LinearRunner`, which records a total of `sample_size`
    /// rounds. The first round records the timing of `round_size_start`
    /// executions of the target. Each subsequent round inclues an additional
    /// `round_size_step` executions.
    pub fn new(
        round_size_start: usize,
        round_size_step: usize,
        sample_size: usize,
    ) -> Self {
        LinearRunner {
            round_size_start,
            round_size_step,
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

        end.since(&start)
    }
}

impl<T: Timer> Runner<Round> for LinearRunner<T> {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<Round>
        where Target: FnMut() -> Ret {

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
